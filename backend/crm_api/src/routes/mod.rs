/*
This file creates the routes.
*/

 mod hello_world;
 mod get_topics;
 use axum::{
     extract::FromRef,
     routing::get,
     Router,
 };
 use sqlx::postgres::PgPool;
 use hello_world::hello_world;
 use get_topics::get_all_topics_handler;


#[derive(Clone, FromRef)]
pub struct AppState {
    pub db_pool: PgPool
 }

pub fn create_routes(db_pool: PgPool) -> Router<> {
    let app_state: AppState = AppState { db_pool };
     Router::new().route("/", get(hello_world))
     .route("/topics", get(get_all_topics_handler))
     .with_state(app_state)
 }
