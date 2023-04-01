/*
This file creates the routes.
*/

 mod hello_world;
 use axum::{
    // what does FromRef do?
     extract::FromRef,
     routing::get,
     Router,
 };
 use sqlx::postgres::PgPool;
 use hello_world::hello_world;

// What's the purpose of this derive statement
#[derive(Clone, FromRef)]
pub struct AppState {
    pub db_pool: PgPoolOptions
 }

pub fn create_routes(db_pool: PgPool) -> Router<> {
    let app_state = AppState { db_pool };
     Router::new().route("/", get(hello_world))

 }
