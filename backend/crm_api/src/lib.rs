mod routes;
use axum::{
    Router,
};
use routes::create_routes;
use sqlx::postgres::PgPoolOptions;


 pub async fn run(db_uri: &str) -> Result<(), sqlx::Error> {

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_uri).await?; // might need to use unwrap() here.

    // build our server/application
    let app: Router = create_routes(pool);

    // run it with hyper on localhost:3000
    // 0.0.0.0 makes it compatible with docker containers
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();

    // create DB connection pool
    // wait this has to be available in the handlers 
    // so maybe I can't define this here.
    
    // expected enum `Result`, found `()`
    Ok(())
 }
