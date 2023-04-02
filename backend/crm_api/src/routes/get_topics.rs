use axum::{
    Extension, Json,
};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, FromRow, Result};


#[derive(Serialize, Deserialize, FromRow)]
pub struct Topic {
    id: i64, 
    topic: String,
}

pub async fn get_all_topics_handler(
    Extension(db_pool): Extension<PgPool>
) -> Json<Vec<Topic>> {

    let topics = get_all_topics(&db_pool).await.unwrap();
    Json(topics)
}

pub async fn get_all_topics(db_pool: &PgPool) -> Result<Vec<Topic>> {
    let topics = sqlx::query_as::<_, Topic>("SELECT id, topic FROM platform.topics")
        .fetch_all(db_pool)
        .await?;

    Ok(topics)
}


