#![deny(missing_docs)]

//! # roux.rs
//! This crate provides simple access to the Reddit API.
//!
//! # Using OAuth
//! To create an OAuth client set `client_id` and `client_secret`:
//! ```no_run
//! use roux::subreddit::Subreddits;
//! use roux::util::RouxError;
//! use roux::RedditBuilder;
//! use std::env::var;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), RouxError> {
//!     let client_id = var("CLIENT_ID").unwrap();
//!     let client_secret = var("CLIENT_SECRET").unwrap();
//!     let reddit = RedditBuilder::new()
//!         .user_agent("roux-demo")
//!         .client_id(&client_id)
//!         .client_secret(&client_secret)
//!         .build()
//!         .await?;
//!
//!     let subreddits = Subreddits::new(&reddit)
//!         .search("cats", Some(50), None)
//!         .await?;
//!
//!     for subreddit in subreddits.data.children.iter() {
//!         println!("{}", subreddit.data.title.as_ref().unwrap());
//!     }
//!     Ok(())
//! }
//! ```
//!
//! # Readonly
//! To create a readonly client, don't set `client_id` or `client_secret`:
//! ```no_run
//! use roux::subreddit::Subreddit;
//! use roux::util::RouxError;
//! use roux::RedditBuilder;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), RouxError> {
//!     let reddit = RedditBuilder::new().build().await?;
//!     let submissions = Subreddit::new(&reddit, "golang").top(100, None).await?;
//!
//!     for submission in submissions.data.children.iter() {
//!         println!("{}", submission.data.title);
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! # Submit A Text Post
//! ```no_run
//! use roux::util::RouxError;
//! use roux::RedditBuilder;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), RouxError> {
//!     let me = RedditBuilder::new()
//!         .user_agent("super-reddit-bot:1.6.3 by /u/blarstacoman")
//!         .username("blarstacoman")
//!         .password("supersecret")
//!         .build()
//!         .await?
//!         .login()
//!         .await?;
//!
//!     me.submit_text("Chicken Katsu Curry", "Chicken katsu recipe", "food").await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! # Submit A Link Post
//! ```no_run
//! use roux::util::RouxError;
//! use roux::RedditBuilder;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), RouxError> {
//!     let me = RedditBuilder::new()
//!         .user_agent("super-reddit-bot:1.6.3 by /u/blarstacoman")
//!         .username("blarstacoman")
//!         .password("supersecret")
//!         .build()
//!         .await?
//!         .login()
//!         .await?;
//!
//!     me.submit_link("Some neato link", "https://neatolink.com", "pics");
//!
//!     Ok(())
//! }
//! ```

use serde::Deserialize;

/// Client configuration module.
mod config;

/// Subreddit module.
pub mod subreddit;
pub use subreddit::{Subreddit, Subreddits};

/// User module.
pub mod user;
pub use user::User;

/// Me module.
pub mod me;
pub use me::Me;

pub mod responses;

/// Utils for requests.
pub mod util;
use util::url;

use crate::util::RouxError;
use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE, USER_AGENT},
    Client as ReqwestClient, ClientBuilder as ReqwestClientBuilder,
};

#[derive(Deserialize, Debug)]
struct AuthData {
    pub access_token: String,
}

/// A HTTP client for making requests to the Reddit API.
pub struct Reddit {
    /// Reqwest client for making HTTP requests.
    pub client: ReqwestClient,
    /// Configuration for our API requests.
    pub config: config::Config,
}

impl Reddit {
    /// Login as a user.
    pub async fn login(self) -> Result<me::Me, RouxError> {
        let url = format!("{}/api/v1/access_token", self.config.url);
        let form = [
            ("grant_type", "password"),
            ("username", &self.config.username.to_owned().unwrap()),
            ("password", &self.config.password.to_owned().unwrap()),
        ];

        let request = self
            .client
            .post(url)
            .header(USER_AGENT, &self.config.user_agent[..])
            .basic_auth(
                &self.config.client_id.clone().unwrap(),
                self.config.client_secret.clone(),
            )
            .form(&form);

        let response = request.send().await?;

        if response.status() == 200 {
            let auth_data = response.json::<AuthData>().await.unwrap();
            Ok(me::Me::new(&auth_data.access_token, self.config))
        } else {
            Err(RouxError::Status(response))
        }
    }
}

/// A builder type for creating configured clients.
pub struct RedditBuilder {
    config: config::Config,
}

impl RedditBuilder {
    /// Create a new client builder.
    pub fn new() -> Self {
        Self {
            config: config::Config::new(),
        }
    }

    /// Set the user agent.
    pub fn user_agent(mut self, user_agent: &str) -> Self {
        self.config.user_agent = user_agent.to_owned();
        self
    }

    /// Set the client id.
    pub fn client_id(mut self, client_id: &str) -> Self {
        self.config.client_id = Some(client_id.to_owned());
        self
    }

    /// Set the client secret.
    pub fn client_secret(mut self, client_secret: &str) -> Self {
        self.config.client_secret = Some(client_secret.to_owned());
        self
    }

    /// Set the username.
    pub fn username(mut self, username: &str) -> Self {
        self.config.username = Some(username.to_owned());
        self
    }

    /// Set the password.
    pub fn password(mut self, password: &str) -> Self {
        self.config.password = Some(password.to_owned());
        self
    }

    /// Build the client.
    pub async fn build<'a>(mut self) -> Result<Reddit, RouxError> {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, self.config.user_agent[..].parse().unwrap());
        headers.insert(ACCEPT, "application/json"[..].parse().unwrap());
        headers.insert(
            CONTENT_TYPE,
            "application/x-www-form-urlencoded"[..].parse().unwrap(),
        );
        let client = ReqwestClientBuilder::new()
            .default_headers(headers)
            .use_rustls_tls()
            .build()
            .unwrap();

        if self.config.client_id.is_none() && self.config.client_secret.is_none() {
            return Ok(Reddit {
                client,
                config: self.config,
            });
        }

        let url = &url::build_url("api/v1/access_token")[..];
        let form = [("grant_type", "client_credentials")];

        let request = client
            .post(url)
            .header(USER_AGENT, &self.config.user_agent[..])
            .basic_auth(
                &self.config.client_id.clone().unwrap(),
                self.config.client_secret.clone(),
            )
            .form(&form);

        let response = request.send().await?;

        if response.status() == 200 {
            let auth_data = response.json::<AuthData>().await.unwrap();
            let mut headers = HeaderMap::new();
            headers.insert(USER_AGENT, self.config.user_agent[..].parse().unwrap());
            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", auth_data.access_token)).unwrap(),
            );
            headers.insert(ACCEPT, "application/json"[..].parse().unwrap());
            headers.insert(CONTENT_TYPE, "application/json"[..].parse().unwrap());

            let subreddit_client = ReqwestClientBuilder::new()
                .default_headers(headers)
                .use_rustls_tls()
                .build()
                .unwrap();

            self.config.url = config::DEFAULT_AUTHENTICATED_URL.to_owned();
            return Ok(Reddit {
                client: subreddit_client,
                config: self.config,
            });
        } else {
            return Err(RouxError::Status(response));
        }
    }
}
