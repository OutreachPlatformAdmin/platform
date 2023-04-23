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
}

#[derive(Deserialize)]
pub struct CreateTopic {
    topic: String,
}

pub async fn get_all_topics_handler(State(db_pool): State<PgPool>) -> Json<Vec<Topic>> {
    let topics = get_all_topics(&db_pool).await.unwrap();
    Json(topics)
}

pub async fn get_all_topics(db_pool: &PgPool) -> Result<Vec<Topic>> {
    let topics = sqlx::query_as::<_, Topic>("SELECT id, topic FROM platform.topics")
        .fetch_all(db_pool)
        .await?;

    Ok(topics)
}

pub async fn new_topic_handler(
    State(db_pool): State<PgPool>,
    Json(payload): Json<CreateTopic>,
) -> Response {
    let topic = &payload.topic;
    let insert_result = query!("INSERT INTO platform.topics (topic) VALUES ($1)", topic)
        .execute(&db_pool)
        .await;
    let insert = match insert_result {
        Ok(_insert_result) => "new topic created".into_response(),
        Err(_err) => (StatusCode::INTERNAL_SERVER_ERROR, "Insertion to DB failed").into_response(),
    };
    insert
}
