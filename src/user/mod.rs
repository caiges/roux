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

use crate::client::Client;
use crate::util::RouxError;

pub mod responses;
use crate::subreddit::responses::{Submissions, SubredditComments};
use responses::Overview;

/// User.
pub struct User<'client> {
    /// User's name.
    pub user: String,
    client: &'client Client,
}

impl<'client> User<'client> {
    /// Create a new `User` instance.
    pub fn new(client: &'client Client, user: &str) -> User<'client> {
        User {
            user: user.to_owned(),
            client,
        }
    }

    /// Get user's overview.
    pub async fn overview(&self) -> Result<Overview, RouxError> {
        Ok(self
            .client
            .client
            .get(&format!(
                "{}/user/{}/overview.json",
                self.client.config.url, self.user
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
}

#[cfg(test)]
mod tests {
    use super::User;
    use crate::client::ClientBuilder;
    use tokio;

    #[tokio::test]
    async fn test_no_auth() {
        let client = ClientBuilder::new().build().await.unwrap();
        let user = User::new(&client, "beneater");

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
