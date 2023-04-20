/*
This file creates the routes.
*/

mod hello_world;
mod terms;
mod topics;
use axum::{extract::FromRef, routing::get, Router};
use hello_world::hello_world;
use sqlx::postgres::PgPool;
use terms::get_all_terms_handler;
use topics::get_all_topics_handler;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub db_pool: PgPool,
}

pub fn create_routes(db_pool: PgPool) -> Router {
    let app_state: AppState = AppState { db_pool };
    Router::new()
        .route("/", get(hello_world))
        .route("/topics", get(get_all_topics_handler))
        .route("/terms", get(get_all_terms_handler))
        .with_state(app_state)
}
