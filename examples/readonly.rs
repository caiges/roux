use ruhroux::subreddit::Subreddit;
use ruhroux::util::RouxError;
use ruhroux::RedditBuilder;

#[tokio::main]
async fn main() -> Result<(), RouxError> {
    let reddit = RedditBuilder::new().build().await?;
    let submissions = Subreddit::new(&reddit, "golang").top(100, None).await?;

    for submission in submissions.data.children.iter() {
        println!("{}", submission.data.title);
    }
    Ok(())
}
