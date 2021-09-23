use roux::client::ClientBuilder;
use std::env::var;

fn main() {
    println!("Hello, world!");
    let client_id = var("CLIENT_ID").unwrap();
    let client_secret = var("CLIENT_SECRET").unwrap();
    let client = ClientBuilder::new()
        .user_agent("roux-demo")
        .client_id(client_id)
        .client_secret(client_secret)
        .build();
    let submissions = client.subreddit("meow").latest();
    println!("{:?}", submissions);
}
