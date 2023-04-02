mod routes;
use axum::{
    Router,
};
use routes::create_routes;
use sqlx::postgres::PgPoolOptions;


 pub async fn run(db_uri: &str) {

    // TODO: replace unwrap with ? and handle potential error response from Result<>
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_uri).await.unwrap(); 

    // build our server/application
    let app: Router = create_routes(pool);

    // run it with hyper on localhost:3000
    // 0.0.0.0 makes it compatible with docker containers
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();
 }
