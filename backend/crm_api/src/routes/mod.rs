/*
This file creates the routes.
*/

mod get_topics;
mod hello_world;
use axum::{extract::FromRef, routing::get, Router};
use get_topics::get_all_topics_handler;
use hello_world::hello_world;
use sqlx::postgres::PgPool;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub db_pool: PgPool,
}

pub fn create_routes(db_pool: PgPool) -> Router {
    let app_state: AppState = AppState { db_pool };
    Router::new()
        .route("/", get(hello_world))
        .route("/topics", get(get_all_topics_handler))
        .with_state(app_state)
}
