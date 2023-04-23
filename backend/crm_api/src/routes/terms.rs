use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use sqlx::{query, FromRow, PgPool, Result};

#[derive(Serialize, Deserialize, FromRow)]
pub struct Term {
    id: i32,
    term: String,
}

#[derive(Deserialize)]
pub struct QueryParams {
    topic: String,
}

pub async fn get_all_terms_handler(State(db_pool): State<PgPool>) -> Json<Vec<Term>> {
    let terms = get_all_terms(&db_pool)
        .await
        .expect("failed to retrieve terms");
    Json(terms)
}

pub async fn get_all_terms(db_pool: &PgPool) -> Result<Vec<Term>> {
    let terms = sqlx::query_as::<_, Term>("SELECT id, term FROM platform.terms")
        .fetch_all(db_pool)
        .await?;

    Ok(terms)
}

/*
Ex1:
http://localhost:3000/terms-from-topic?topic=new%20topic
Ex2:
http://localhost:3000/terms-from-topic?topic=neoliberalism
 */
pub async fn get_all_terms_for_topic_handler(
    State(db_pool): State<PgPool>,
    params: axum::extract::Query<QueryParams>,
) -> Json<Vec<Term>> {
    let terms = get_all_terms_for_a_topic(&db_pool, &params.topic)
        .await
        .expect("failed ot retrieve terms for a given topic");
    Json(terms)
}

pub async fn get_all_terms_for_a_topic(db_pool: &PgPool, topic: &str) -> Result<Vec<Term>> {
    // first get topic id
    let record = query!("SELECT id from platform.topics where topic = $1", topic)
        .fetch_one(db_pool)
        .await?;

    let terms: Vec<Term> = sqlx::query_as!(
        Term,
        "SELECT id, term FROM platform.terms as terms 
        INNER JOIN platform.terms_to_topics as terms_to_topics on 
        terms.id = terms_to_topics.term_id 
        where terms_to_topics.topic_id = $1",
        record.id
    )
    .fetch_all(db_pool)
    .await?;

    Ok(terms)
}
