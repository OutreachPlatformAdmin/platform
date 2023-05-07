use std::os::windows::process;

use serde::Deserialize;
use sqlx::{query, FromRow, PgPool, Result};

pub fn process_optional_param(param: Option<Vec<String>>) -> Vec<String> {
    let mut processed_param = vec![];
    if let Some(populated_param) = param {
        processed_param = populated_param
    }
    processed_param
}

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

pub async fn insert_topic_or_term(
    payload: CreateTopicOrTerm,
    topic_or_term: &str,
    db_pool: &PgPool,
) -> Result<()> {
    let bullet_points = process_optional_param(payload.bullet_points);
    let examples = process_optional_param(payload.examples);
    let parallels = process_optional_param(payload.parallels);
    let ai_bullet_points = process_optional_param(payload.ai_bullet_points);
    let ai_parallels = process_optional_param(payload.ai_parallels);
    let ai_examples = process_optional_param(payload.ai_examples);

    let query_string = format!("INSERT INTO platform.{}s ({}, is_verified, brief_description, full_description, 
        bullet_points, examples, parallels, ai_brief_description, ai_full_description, ai_bullet_points, ai_parallels, 
        ai_examples) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)", topic_or_term, topic_or_term);

    let _insert_result = sqlx::query(&query_string)
        .bind(payload.value)
        .bind(payload.is_verified)
        .bind(payload.brief_description)
        .bind(payload.full_description)
        .bind(bullet_points.as_slice())
        .bind(examples.as_slice())
        .bind(parallels.as_slice())
        .bind(payload.ai_brief_description)
        .bind(payload.ai_full_description)
        .bind(ai_bullet_points.as_slice())
        .bind(ai_parallels.as_slice())
        .bind(ai_examples.as_slice())
        .execute(db_pool)
        .await;

    Ok(())
}

pub async fn build_bridge_tables(
    payload: CreateTopicOrTerm,
    entity_type: &str,
    db_pool: &PgPool,
) -> Result<()> {
    // first use payload.value to query that table and get the id for the value
    let get_id_query_str = format!(
        "SELECT id from platform.{}s where {} = $1",
        entity_type, entity_type
    );

    let record = sqlx::query(&get_id_query_str)
        .bind(payload.value)
        .fetch_one(db_pool)
        .await?;

    // you can then access the id via record.id


    let related_terms = process_optional_param(payload.related_terms);
    let related_topics = process_optional_param(payload.related_topics);
    let related_sources = process_optional_param(payload.related_sources);

    // self-referential data currently not supported for terms
    if !related_terms.is_empty() && entity_type != "term" {
        // build a SQL query that puts terms in the IN statement
        // this should return Result<Vec<?>> -- we have to define the struct... or wait this is query so it will be PgRow
        let ids = query!("SELECT id from platform.terms where term in ($1)", related_terms.as_slice()).fetch_all(db_pool).await?;
    }
    // now get ids of the optional array values 
    if let Some(related_terms) = payload.related_terms {
            // you can add in additonal if statements here if needed...
            // build a SQL query that puts terms in the IN statement
            let ids = query!("SELECT id from platform.terms where term in ($1)", related_terms.as_slice())
    }


    Ok(())
}
