use crm_api::run;
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_uri = env::var("DATABASE_URL").unwrap();
    run(&db_uri).await
}
