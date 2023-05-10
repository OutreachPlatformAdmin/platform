use crate::helpers::handler_utils::{build_bridge_tables, insert_topic_or_term, CreateTopicOrTerm};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Result};

#[derive(Serialize, Deserialize, FromRow)]
pub struct Topic {
    id: i32,
    topic: String,
    is_verified: bool,
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

/*
 /topics
- returns all topics
 */
pub async fn get_all_topics_handler(State(db_pool): State<PgPool>) -> Response {
    let topics = get_all_topics(&db_pool).await;
    match topics {
        Ok(topics) => (StatusCode::OK, Json(topics)).into_response(),
        // for errors Axum expects the axum::response::Response type
        // example output: error returned from database: relation "platform.tipics" does not exist
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response(),
    }
}

pub async fn get_all_topics(db_pool: &PgPool) -> Result<Vec<Topic>> {
    let topics = sqlx::query_as::<_, Topic>(
        "SELECT id, topic, is_verified, brief_description,
    full_description, bullet_points, examples, parallels, ai_brief_description, ai_full_description,
    ai_bullet_points, ai_parallels, ai_examples
    FROM platform.topics",
    )
    .fetch_all(db_pool)
    .await?;
    Ok(topics)
}

/*
/new-topic
Body:
{
   "topic": "<new_topic_name>"
}
*/
pub async fn new_topic_handler(
    State(db_pool): State<PgPool>,
    Json(payload): Json<CreateTopicOrTerm>,
) -> Response {
    let insert_result = insert_topic_or_term(&payload, "topic", &db_pool).await;
    // todo: look up how I can do error handling for both of these function calls since they both return Result
    let _other_insert_result = build_bridge_tables(&payload, "topic", &db_pool).await;
    match insert_result {
        Ok(_insert_result) => "new topic created".into_response(),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response(),
    }
}
