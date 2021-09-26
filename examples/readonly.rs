use roux::client::ClientBuilder;
use roux::subreddit::Subreddit;
use roux::util::RouxError;

#[tokio::main]
async fn main() -> Result<(), RouxError> {
    let client = ClientBuilder::new().build().await?;
    let submissions = Subreddit::new(&client, "golang").top(100, None).await?;

    for submission in submissions.data.children.iter() {
        println!("{}", submission.data.title);
    }
    Ok(())
}
