use crate::helpers::shared_types::CreateSource;
use lazy_static::lazy_static;
use serde::Deserialize;
use sqlx::{FromRow, PgPool, Result};
use std::collections::HashMap;

#[derive(Deserialize, FromRow)]
pub struct CreateTopicOrTerm {
    name: String,
    is_verified: Option<bool>,
    brief_description: Option<String>,
    full_description: Option<String>,
    bullet_points: Option<Vec<String>>,
    examples: Option<Vec<String>>,
    parallels: Option<Vec<String>>,
    ai_brief_description: Option<String>,
    ai_full_description: Option<String>,
    ai_bullet_points: Option<Vec<String>>,
    ai_parallels: Option<Vec<String>>,
    ai_examples: Option<Vec<String>>,
    related_terms: Option<Vec<String>>,
    related_topics: Option<Vec<String>>,
    related_sources: Option<Vec<String>>,
}

#[derive(Deserialize, FromRow)]
pub struct IdRow {
    id: i32,
}

pub trait CreateEntity {
    fn name(&self) -> &String;
    fn related_terms(&self) -> &Option<Vec<String>>;
    fn related_topics(&self) -> &Option<Vec<String>>;
    fn related_sources(&self) -> &Option<Vec<String>>;
}

impl CreateEntity for CreateTopicOrTerm {
    fn name(&self) -> &String {
        &self.name
    }

    fn related_terms(&self) -> &Option<Vec<String>> {
        &self.related_terms
    }

    fn related_topics(&self) -> &Option<Vec<String>> {
        &self.related_topics
    }

    fn related_sources(&self) -> &Option<Vec<String>> {
        &self.related_sources
    }
}

impl CreateEntity for CreateSource {
    fn name(&self) -> &String {
        &self.name
    }

    fn related_terms(&self) -> &Option<Vec<String>> {
        &self.related_terms
    }

    fn related_topics(&self) -> &Option<Vec<String>> {
        &self.related_topics
    }

    fn related_sources(&self) -> &Option<Vec<String>> {
        &self.related_sources
    }
}

pub fn process_optional_vec(param: &Option<Vec<String>>) -> Vec<String> {
    let mut processed_param = vec![];
    if let Some(populated_param) = param {
        // TODO: come back and update this method to output a reference instead
        processed_param = populated_param.clone()
    }
    processed_param
}

// The lazy_static macro ensures that the HashMap is initialized lazily at runtime, which means that it's only created when it's first accessed.
lazy_static! {
    static ref BRIDGE_TABLES: HashMap<&'static str, HashMap<&'static str, &'static str>> = {
        let mut data = HashMap::new();
        data.insert(
            "topic",
            HashMap::from([("term", "terms_to_topics"), ("source", "topics_to_sources")]),
        );
        data.insert(
            "term",
            HashMap::from([("topic", "terms_to_topics"), ("source", "terms_to_sources")]),
        );
        data.insert(
            "source",
            HashMap::from([("term", "terms_to_sources"), ("topic", "topics_to_sources")]),
        );
        data
    };
}

pub async fn insert_topic_or_term(
    payload: &CreateTopicOrTerm,
    topic_or_term: &str,
    db_pool: &PgPool,
) -> Result<()> {
    let bullet_points = process_optional_vec(&payload.bullet_points);
    let examples = process_optional_vec(&payload.examples);
    let parallels = process_optional_vec(&payload.parallels);
    let ai_bullet_points = process_optional_vec(&payload.ai_bullet_points);
    let ai_parallels = process_optional_vec(&payload.ai_parallels);
    let ai_examples = process_optional_vec(&payload.ai_examples);

    let query_string = format!("INSERT INTO platform.{}s ({}, is_verified, brief_description, full_description, 
        bullet_points, examples, parallels, ai_brief_description, ai_full_description, ai_bullet_points, ai_parallels, 
        ai_examples) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)", topic_or_term, topic_or_term);

    let _insert_result = sqlx::query(&query_string)
        .bind(&payload.name)
        .bind(&payload.is_verified)
        .bind(&payload.brief_description)
        .bind(&payload.full_description)
        .bind(bullet_points.as_slice())
        .bind(examples.as_slice())
        .bind(parallels.as_slice())
        .bind(&payload.ai_brief_description)
        .bind(&payload.ai_full_description)
        .bind(ai_bullet_points.as_slice())
        .bind(ai_parallels.as_slice())
        .bind(ai_examples.as_slice())
        .execute(db_pool)
        .await;

    Ok(())
}

// payload can either be CreateTopicOrTerm or CreateSource
// in order for &T to work, I need to define shared behavior:
// each struct has .name, .related_sources, .related_terms, .related_topics fields
pub async fn build_bridge_tables<T: CreateEntity>(
    payload: &T,
    entity_type: &str,
    db_pool: &PgPool,
) -> Result<()> {
    // first use payload.value to query that table and get the id for the value
    let get_id_query_str: String;
    if entity_type == "source" {
        get_id_query_str = format!("SELECT id from platform.{}s where name = $1", entity_type);
    } else {
        get_id_query_str = format!(
            "SELECT id from platform.{}s where {} = $1",
            entity_type, entity_type
        );
    }
    let entity_row = sqlx::query_as::<_, IdRow>(&get_id_query_str)
        .bind(&payload.name())
        .fetch_one(db_pool)
        .await?;

    let related_terms = process_optional_vec(&payload.related_terms());
    let related_terms_str = related_terms.join(",");
    let related_topics = process_optional_vec(&payload.related_topics());
    let related_topics_str = related_topics.join(",");
    // TODO: implement sources logic.
    let related_sources = process_optional_vec(&payload.related_sources());
    let related_sources_str = related_sources.join(",");
    let term_ids: Vec<i32>;
    let topic_ids: Vec<i32>;
    let source_ids: Vec<i32>;

    // self-referential data currently not supported for terms
    if !related_terms.is_empty() && entity_type != "term" {
        // term_id_rows is of type Vec<IdRow>
        if let Ok(term_id_rows) = sqlx::query_as!(
            IdRow,
            "SELECT id from platform.terms where term in ($1)",
            related_terms_str
        )
        .fetch_all(db_pool)
        .await
        {
            term_ids = term_id_rows.iter().map(|row| row.id).collect();
            let bridge_table: &str;
            if let Some(inner_hashmap) = BRIDGE_TABLES.get(entity_type) {
                if let Some(table_name) = inner_hashmap.get("term") {
                    bridge_table = table_name;
                    let insert_query_str = format!(
                        "INSERT INTO platform.{} (term_id, {}_id) VALUES ($1, {})",
                        bridge_table, entity_type, entity_row.id
                    );
                    for term_id in &term_ids {
                        sqlx::query(&insert_query_str)
                            .bind(term_id)
                            .execute(db_pool)
                            .await?;
                    }
                }
            }
        }
    }
    // TODO: probably should create another helper function since a lot of this logic is duplicated
    // TODO: add support for adding self-referential topics
    if !related_topics.is_empty() && entity_type != "topic" {
        if let Ok(topic_id_rows) = sqlx::query_as!(
            IdRow,
            "SELECT id from platform.topics where topic in ($1)",
            related_topics_str
        )
        .fetch_all(db_pool)
        .await
        {
            topic_ids = topic_id_rows.iter().map(|row| row.id).collect();
            let bridge_table: &str;
            if let Some(inner_hashmap) = BRIDGE_TABLES.get(entity_type) {
                if let Some(table_name) = inner_hashmap.get("topic") {
                    bridge_table = table_name;
                    let insert_query_str = format!(
                        "INSERT INTO platform.{} (topic_id, {}_id) VALUES ($1, {})",
                        bridge_table, entity_type, entity_row.id
                    );
                    for topic_id in &topic_ids {
                        sqlx::query(&insert_query_str)
                            .bind(topic_id)
                            .execute(db_pool)
                            .await?;
                    }
                }
            }
        }
    }
    if !related_sources.is_empty() && entity_type != "source" {
        if let Ok(source_id_rows) = sqlx::query_as!(
            IdRow,
            "SELECT id from platform.sources where name in ($1)",
            related_sources_str
        )
        .fetch_all(db_pool)
        .await
        {
            source_ids = source_id_rows.iter().map(|row| row.id).collect();
            let bridge_table: &str;
            if let Some(inner_hashmap) = BRIDGE_TABLES.get(entity_type) {
                if let Some(table_name) = inner_hashmap.get("source") {
                    bridge_table = table_name;
                    let insert_query_str = format!(
                        "INSERT INTO platform.{} (source_id, {}_id) VALUES ($1, {})",
                        bridge_table, entity_type, entity_row.id
                    );
                    for source_id in &source_ids {
                        sqlx::query(&insert_query_str)
                            .bind(source_id)
                            .execute(db_pool)
                            .await?;
                    }
                }
            }
        }
    }

    Ok(())
}
