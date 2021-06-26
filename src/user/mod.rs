//! # User
//! A read-only module to read data from for a specific user.
//!
//! # Usage
//! ```rust
//! use roux::User;
//! use tokio;
//!
//! #[tokio::main]
//! async fn main() {
//!     let user = User::new("beanpup_py");
//!     // Now you are able to:
//!
//!     // Get overview
//!     let overview = user.overview().await;
//!
//!     // Get submitted posts.
//!     let submitted = user.submitted().await;
//!
//!     // Get comments.
//!     let comments = user.comments().await;
//! }
//! ```

extern crate reqwest;
extern crate serde_json;

use crate::util::RouxError;
use futures::stream;
use futures::stream::{Stream, StreamExt};
use reqwest::Client;
use std::iter::Iterator;
use std::pin::Pin;
pub mod responses;
use crate::requests::AfterState;
use crate::subreddit::responses::{Submissions, SubmissionsData, SubredditComments};
use responses::Overview;
/// User.
pub struct User {
    /// User's name.
    pub user: String,
    client: Client,
}

impl User {
    /// Create a new `User` instance.
    pub fn new(user: &str) -> User {
        User {
            user: user.to_owned(),
            client: Client::new(),
        }
    }

    /// Get user's overview.
    pub async fn overview(&self) -> Result<Overview, RouxError> {
        Ok(self
            .client
            .get(&format!(
                "https://www.reddit.com/user/{}/overview/.json",
                self.user
            ))
            .send()
            .await?
            .json::<Overview>()
            .await?)
    }

    /// Get user's submitted posts.
    pub async fn submitted(&self) -> Result<Submissions, RouxError> {
        Ok(self
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
                        "https://www.reddit.com/user/{}/submitted/.json?after={}",
                        self.user, a
                    ),
                    None => format!("https://www.reddit.com/user/{}/submitted/.json", self.user),
                };

                match self.client.get(&url).send().await {
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
                        Err(_) => None,
                    },
                    Err(_) => None,
                }
            })
            .flatten(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::User;
    use tokio;

    #[tokio::test]
    async fn test_no_auth() {
        let user = User::new("beneater");

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
