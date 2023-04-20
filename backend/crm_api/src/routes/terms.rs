use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Result};

#[derive(Serialize, Deserialize, FromRow)]
pub struct Term {
    id: i32,
    term: String,
}

pub async fn get_all_terms_handler(State(db_pool): State<PgPool>) -> Json<Vec<Term>> {
    let terms = get_all_terms(&db_pool).await.unwrap();
    Json(terms)
}

pub async fn get_all_terms(db_pool: &PgPool) -> Result<Vec<Term>> {
    let terms = sqlx::query_as::<_, Term>("SELECT id, term FROM platform.terms")
        .fetch_all(db_pool)
        .await?;

    Ok(terms)
}

pub async fn get_all_terms_for_a_topic(db_pool: &PgPool, term: &str) -> Result<Vec<Term>> {
    // first get topic id 
    let topic_id = sqlx::query!("SELECT id from platform.topics where topic = ?", term)
    .fetch_one(db_pool)
    .await?;



    let terms: sqlx::query_as::<_, Term>("SELECT id, term FROM platform.terms as terms
    JOIN platform.terms_to_topics as terms_to_topics
     on terms.id = terms_to_topics.term_id 
     where")
    .fetch_all(db_pool)
    .await?;
}



