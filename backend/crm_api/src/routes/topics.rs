use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::{query, FromRow, PgPool, Result};

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

#[derive(Deserialize, FromRow)]
pub struct CreateTopic {
    topic: String,
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
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response()
    }
}

pub async fn get_all_topics(db_pool: &PgPool) -> Result<Vec<Topic>> {
    let topics = sqlx::query_as::<_, Topic>("SELECT id, topic, is_verified, brief_description,
    full_description, bullet_points, examples, parallels, ai_brief_description, ai_full_description,
    ai_bullet_points, ai_parallels, ai_examples
    FROM platform.topics")
        .fetch_all(db_pool)
        .await?;
    Ok(topics)
}

pub fn process_optional_param(param: Option<Vec<String>>) -> Vec<String> {
    let mut processed_param = vec![];
    if let Some(populated_param) = param {
        processed_param = populated_param
    }
    processed_param
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
    Json(payload): Json<CreateTopic>,
) -> Response {
    let bullet_points = process_optional_param(payload.bullet_points);
    let examples = process_optional_param(payload.examples);
    let parallels = process_optional_param(payload.parallels);
    let ai_bullet_points = process_optional_param(payload.ai_bullet_points);
    let ai_parallels = process_optional_param(payload.ai_parallels);
    let ai_examples = process_optional_param(payload.ai_examples);

    let insert_query = query!("INSERT INTO platform.topics (topic, is_verified, brief_description, full_description, 
        bullet_points, examples, parallels, ai_brief_description, ai_full_description, ai_bullet_points, ai_parallels, 
        ai_examples) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)",
    payload.topic,
    payload.is_verified,
    payload.brief_description,
    payload.full_description,
    bullet_points.as_slice(),
    examples.as_slice(),
    parallels.as_slice(),
    payload.ai_brief_description,
    payload.ai_full_description,
    ai_bullet_points.as_slice(),
    ai_parallels.as_slice(),
    ai_examples.as_slice(),
    );
    let insert_result = insert_query.execute(&db_pool).await;
    match insert_result {
        Ok(_insert_result) => "new topic created".into_response(),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response(),
    }
}
