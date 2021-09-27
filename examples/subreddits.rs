use roux::subreddit::Subreddits;
use roux::util::RouxError;
use roux::RedditBuilder;
use std::env::var;

#[tokio::main]
async fn main() -> Result<(), RouxError> {
    let client_id = var("CLIENT_ID").unwrap();
    let client_secret = var("CLIENT_SECRET").unwrap();
    let reddit = RedditBuilder::new()
        .user_agent("roux-demo")
        .client_id(&client_id)
        .client_secret(&client_secret)
        .build()
        .await
        .expect("could not build client");

    let subreddits = Subreddits::new(&reddit)
        .search("cats", Some(50), None)
        .await?;

    for subreddit in subreddits.data.children.iter() {
        println!("{}", subreddit.data.title.as_ref().unwrap());
    }
    Ok(())
}
