use serde::Deserialize;
use sqlx::{FromRow, PgPool, Result};

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

    let insert_result = sqlx::query(&query_string)
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
