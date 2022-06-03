use crate::subreddit::responses::{Submissions, SubredditComments, SubmissionsData};
use crate::util::RouxError;
use crate::Reddit;
use crate::requests::AfterState;
use futures::stream;
use futures::stream::{Stream, StreamExt};
use std::pin::Pin;
use responses::Overview;

pub mod responses;

/// User.
pub struct User<'reddit> {
    /// User's name.
    pub user: String,
    reddit: &'reddit Reddit,
}

impl<'reddit> User<'reddit> {
    /// Create a new `User` instance.
    pub fn new(reddit: &'reddit Reddit, user: &str) -> User<'reddit> {
        User {
            user: user.to_owned(),
            reddit,
        }
    }

    /// Get user's overview.
    pub async fn overview(&self) -> Result<Overview, RouxError> {
        Ok(self
            .reddit
            .client
            .get(&format!(
                "{}/user/{}/overview.json",
                self.reddit.config.url, self.user
            ))
            .send()
            .await?
            .json::<Overview>()
            .await?)
    }

    /// Get user's submitted posts.
    pub async fn submitted(&self) -> Result<Submissions, RouxError> {
        Ok(self
            .reddit
            .client
            .get(&format!(
                "https://www.reddit.com/user/{}/submitted/.json",
                self.user
            ))
            .send()
            .await?
            .json::<Submissions>()
            .await?)
    }

    /// Get user's submitted comments.
    pub async fn comments(&self) -> Result<SubredditComments, RouxError> {
        Ok(self
            .reddit
            .client
            .get(&format!(
                "https://www.reddit.com/user/{}/comments/.json",
                self.user
            ))
            .send()
            .await?
            .json::<SubredditComments>()
            .await?)
    }

    /// items returns a stream for user submissions.
    pub fn items(&self, after: AfterState) -> Pin<Box<dyn Stream<Item = SubmissionsData> + '_>> {
        Box::pin(
            stream::unfold(after, move |state| async move {
                let af = match state {
                    AfterState::Start(a) => a,
                    AfterState::Next(a) => Some(a),
                    AfterState::End => return None,
                };

                let url = match af {
                    Some(a) => format!(
                        "https://oauth.reddit.com/user/{}/submitted.json?after={}",
                        self.user, a
                    ),
                    None => format!("https://oauth.reddit.com/user/{}/submitted.json", self.user),
                };

                match self.reddit.client.get(&url).send().await {
                    Ok(r) => match r.json::<Submissions>().await {
                        Ok(subs) => {
                            let next_after = match subs.data.after {
                                Some(a) => AfterState::Next(a),
                                None => AfterState::End,
                            };

                            Some((
                                stream::iter(subs.data.children.into_iter().map(move |c| c.data)),
                                next_after,
                            ))
                        }
                        Err(e) => {
                            println!("err getting submissions: {}", e);
                            None
                        },
                    },
                    Err(_) => None,
                }
            }).flatten(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::User;
    use crate::RedditBuilder;
    use tokio;

    #[tokio::test]
    async fn test_no_auth() {
        let reddit = RedditBuilder::new().build().await.unwrap();
        let user = User::new(&reddit, "beneater");

        // Test overview
        let overview = user.overview().await;
        assert!(overview.is_ok());

        // Test submitted
        let submitted = user.submitted().await;
        assert!(submitted.is_ok());

        // Test comments
        let comments = user.comments().await;
        assert!(comments.is_ok());
    }
}
