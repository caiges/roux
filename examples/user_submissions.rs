use futures::stream::StreamExt;
use ruhroux::requests::AfterState;
use ruhroux::user::User;
use ruhroux::util::RouxError;
use ruhroux::RedditBuilder;
use std::env::var;

#[tokio::main]
async fn main() -> Result<(), RouxError> {
    let client_id = var("CLIENT_ID").unwrap();
    let client_secret = var("CLIENT_SECRET").unwrap();

    let reddit = RedditBuilder::new()
        .user_agent("ruhroux-demo")
        .client_id(&client_id)
        .client_secret(&client_secret)
        .build()
        .await?;

    let user = User::new(&reddit, "wil");

    user.items(AfterState::Start(None))
        .for_each(|sub| async move {
            println!("submission: {}", sub.title);
        })
        .await;

    Ok(())
}
