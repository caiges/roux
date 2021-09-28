#[cfg(test)]
mod tests {

    use roux::{util::RouxError, RedditBuilder};
    use tokio;

    static USER_AGENT: &str = "super-reddit-bot:1.6.3 by /u/blarstacoman";

    #[tokio::test]
    async fn test_oauth() -> Result<(), RouxError> {
        let client_id = dotenv::var("CLIENT_ID").unwrap();
        let client_secret = dotenv::var("CLIENT_SECRET").unwrap();
        let username = dotenv::var("USERNAME").unwrap();
        let password = dotenv::var("PASSWORD").unwrap();

        let reddit = RedditBuilder::new()
            .user_agent(USER_AGENT)
            .client_id(&client_id)
            .client_secret(&client_secret)
            .username(&username)
            .password(&password)
            .build()
            .await?
            .login()
            .await;

        match reddit {
            Err(e) => {
                println!("{}", e);
                assert!(false, "failed to login");
            }
            _ => (),
        }
        //assert!(reddit.is_ok());

        //let me = reddit.unwrap();

        //assert!(me.me().await.is_ok());

        Ok(())
    }
}
