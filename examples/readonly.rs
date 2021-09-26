use log::{debug, error, info, log_enabled, Level};
use roux::client::ClientBuilder;
use roux::subreddit::Subreddit;
use roux::util::RouxError;

#[tokio::main]
async fn main() -> Result<(), RouxError> {
    env_logger::init();

    let client = ClientBuilder::new().user_agent("roux-demo").build().await?;

    let submissions = Subreddit::new(&client, "golang").top(50, None).await?;

    for submission in submissions.data.children.iter() {
        println!("{}", submission.data.title);
    }
    Ok(())
}
