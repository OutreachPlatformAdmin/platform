use std::error::Error;
use reqwest::Client;
use std::env;

// tokio runtime for async code
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let api_key = env::var("CIVIC_GOOGLE_API_KEY").unwrap();
    let client = Client::new();
    // this returns the active elections and their election IDs
    // the ocdDivisionId shows you the area the election is for
    let url = format!("https://www.googleapis.com/civicinfo/v2/elections?key={}", api_key);
    let response = client.get(&url).send().await?;
    let body = response.text().await?;
    println!("{}", body);
    let address = "Louisiana";
    let election_id = 8019;
    let url = format!("https://www.googleapis.com/civicinfo/v2/voterinfo?key={}&electionId={}&address={}", api_key, election_id, address);
    let response = client.get(&url).send().await?;
    let body = response.text().await?;
    println!("{}", body);
    Ok(())
}