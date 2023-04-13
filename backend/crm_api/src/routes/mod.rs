/*
This file creates the routes.
*/

mod topics;
mod hello_world;
use axum::{extract::FromRef, routing::get, routing::post, Router};
use hello_world::hello_world;
use sqlx::postgres::PgPool;
use topics::{add_new_topic_handler, get_all_topics_handler};

#[derive(Clone, FromRef)]
pub struct AppState {
    pub db_pool: PgPool,
}

pub fn create_routes(db_pool: PgPool) -> Router {
    let app_state: AppState = AppState { db_pool };
    Router::new()
        .route("/", get(hello_world))
        .route("/topics", get(get_all_topics_handler))
        .route("/new-topic", post(add_new_topic_handler))
        .with_state(app_state)
}
