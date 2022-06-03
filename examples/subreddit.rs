use ruhroux::subreddit::Subreddit;
use ruhroux::util::RouxError;
use ruhroux::RedditBuilder;
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

    let submissions = Subreddit::new(&reddit, "meow").latest(50, None).await?;

    for submission in submissions.data.children.iter() {
        println!("{}", submission.data.title);
    }

    Ok(())
}
