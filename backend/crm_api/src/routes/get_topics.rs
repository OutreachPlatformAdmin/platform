use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Extension, Json,
};
use serde::{Deserialize, Serialize};
use sqlx::{PgRow, PgPool, FromRow, Result};


#[derive(Serialize, Deserialize)]
struct Topic {
    id: i64, 
    topic: String,
}

impl FromRow<PgRow> for Topic {
    fn from_row(row: &PgRow) -> Result<Self> {
        Ok(Topic {
            id: row.try_get("id")?,
            topic: row.try_get("topic")?,
        })
    }
}

pub async fn get_all_topics_handler(
    Extension(db_pool): Extension<PgPool>
) -> Json<Vec<Topic>> {

    let topics = get_all_topics(&db_pool).await.unwrap();
    Ok(Json(topic_result))
}

pub async fn get_all_topics(db_pool: &PgPool) -> Result<Vec<Topic>> {
    let topics = sqlx::query_as::<_, Topic>("SELECT id, topic FROM platform.topics")
        .fetch_all(db_pool)
        .await?;

    Ok(topics)
}


