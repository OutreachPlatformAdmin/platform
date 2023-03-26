mod routes;
use routes::create_routes;
use axum::{
    Router,
};

 pub async fn run() {
    // build our server/application
    let app: Router = create_routes();

    // run it with hyper on localhost:3000
    // 0.0.0.0 makes it compatible with docker containers
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();
 }
