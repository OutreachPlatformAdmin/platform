use std::os::windows::process;

use lazy_static::lazy_static;
use serde::Deserialize;
use sqlx::{postgres::PgRow as Row, query, FromRow, PgPool, Record, Result};
use std::collections::HashMap;

#[derive(Deserialize, FromRow)]
pub struct CreateTopicOrTerm {
    value: String,
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

pub fn process_optional_param(param: &Option<Vec<String>>) -> Vec<String> {
    let mut processed_param = vec![];
    if let Some(populated_param) = param {
        processed_param = *populated_param
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
        data
    };
}

pub async fn insert_topic_or_term(
    payload: &CreateTopicOrTerm,
    topic_or_term: &str,
    db_pool: &PgPool,
) -> Result<()> {
    let bullet_points = process_optional_param(&payload.bullet_points);
    let examples = process_optional_param(&payload.examples);
    let parallels = process_optional_param(&payload.parallels);
    let ai_bullet_points = process_optional_param(&payload.ai_bullet_points);
    let ai_parallels = process_optional_param(&payload.ai_parallels);
    let ai_examples = process_optional_param(&payload.ai_examples);

    let query_string = format!("INSERT INTO platform.{}s ({}, is_verified, brief_description, full_description, 
        bullet_points, examples, parallels, ai_brief_description, ai_full_description, ai_bullet_points, ai_parallels, 
        ai_examples) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)", topic_or_term, topic_or_term);

    let _insert_result = sqlx::query(&query_string)
        .bind(&payload.value)
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

pub async fn build_bridge_tables(
    payload: &CreateTopicOrTerm,
    entity_type: &str,
    db_pool: &PgPool,
) -> Result<()> {
    /*
    Example query: "What are the terms for this topic"
     */

    // first use payload.value to query that table and get the id for the value
    let get_id_query_str = format!(
        "SELECT id from platform.{}s where {} = $1",
        entity_type, entity_type
    );
    let entity_row = sqlx::query_as::<_, IdRow>(&get_id_query_str)
        .bind(&payload.value)
        .fetch_one(db_pool)
        .await?;
    // you can then access the id via record.id

    let related_terms = process_optional_param(&payload.related_terms);
    let related_topics = process_optional_param(&payload.related_topics);
    let related_sources = process_optional_param(&payload.related_sources);
    let term_id_rows: Vec<IdRow>;
    let term_ids: Vec<i32>;

    // self-referential data currently not supported for terms
    if !related_terms.is_empty() && entity_type != "term" {
        // build a SQL query that puts terms in the IN statement

        // this returns a Result<Vec<PgRow>>
        if let Ok(term_id_rows) = sqlx::query_as!(
            IdRow,
            "SELECT id from platform.terms where term in ($1)",
            related_terms.as_slice()
        )
        .fetch_all(db_pool)
        .await
        {
            term_ids = term_id_rows.iter().map(|row| row.id).collect();
        }

        let bridge_table: &str;
        if let Some(inner_hashmap) = BRIDGE_TABLES.get(entity_type) {
            if let Some(table_name) = inner_hashmap.get("term") {
                bridge_table = table_name;
            }
        }

        let mut insert_query_str = format!(
            "INSERT INTO platform.{} (term_id, {}_id) VALUES ",
            bridge_table, entity_type
        );

        let mut param_index = 1;
        for term_id in term_ids {
            insert_query_str.push_str(&format!("(${}, {}),", param_index, entity_row.id));
            param_index += 1;
        }

        // Remove the trailing comma and execute the INSERT statement
        insert_query_str.pop();
        sqlx::query(&insert_query_str)
            .bind(&term_ids)
            .execute(db_pool)
            .await?;
    }

    Ok(())
}
