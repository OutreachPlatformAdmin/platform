use axum::{extract::State, Json};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::{query, Error, FromRow, PgPool, Result};

#[derive(Serialize, Deserialize, FromRow)]
pub struct Topic {
    id: i32,
    topic: String,
}

#[derive(Deserialize)]
struct NewTopic {
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

pub async fn add_new_topic_handler(
    State(db_pool): State<PgPool>,
    new_topic: axum::extract::Json<NewTopic>,
) {
    add_new_topic(&db_pool, &new_topic.topic);
}

// TODO: add permissions
// TODO: add string sanition to prevent SQL injection
pub async fn add_new_topic(db_pool: &PgPool, topic: &str) {
    // this returns sqlx::Result<DB::QueryResult>
    let rows_affected = sqlx::query!("INSERT INTO platform_topics (topic) VALUES (?)", topic)
        .execute(db_pool)
        .await?;
}
