use reqwest::{header::USER_AGENT, Client};
use tokio;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = Client::new();
    let res = client
        .get("https://www.reddit.com/r/rust/hot.json?limit=10")
        .header(USER_AGENT, "reddit-api-test (by u/OkAstronomer5277)")
        .send()
        .await?
        .text()
        .await?;
    println!("body = {:?}", res);
    Ok(())
}
