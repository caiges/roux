use roux::client::ClientBuilder;
use std::env::var;

fn main() {
    let client_id = var("CLIENT_ID").unwrap();
    let client_secret = var("CLIENT_SECRET").unwrap();
    let client = ClientBuilder::new()
        .user_agent("roux-demo")
        .client_id(client_id)
        .client_secret(client_secret)
        .build();
    let subreddits = client.subreddits().search("cats");
    println!("{:?}", subreddits);
}
