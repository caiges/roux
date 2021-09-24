use roux::client::ClientBuilder;
use std::env::var;

fn main() {
    let client = ClientBuilder::new().user_agent("roux-demo").build().await;
    let submissions = client.subreddit("meow").latest();
    println!("{:?}", submissions);
}
