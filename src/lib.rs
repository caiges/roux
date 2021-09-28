#![deny(missing_docs)]

//! # roux.rs
//! This crate provides simple access to the Reddit API.
//!
//! # Using OAuth
//! To create an OAuth client set `client_id` and `client_secret`:
//! ```no_run
//! use roux::Reddit;
//! # use tokio_test;
//!
//! # tokio_test::block_on(async {
//! let client = Reddit::new("USER_AGENT", "CLIENT_ID", "CLIENT_SECRET")
//!     .client_id("your-client-id")
//!     .client_secret("your-client-secret")
//!     .build()
//!     .await;
//!
//! let me = client.unwrap();
//! # })
//! ```
//! It is important that you pick a good user agent. The ideal format is
//! `platform:program:version (by /u/yourname)`, e.g. `macos:roux:v0.3.0 (by /u/beanpup_py)`.
//!
//! This will authticate you as the user given in the username function.
//!
//! # Readonly
//! To create a readonly client, don't set `client_id` or `client_secret`:
//! ```no_run
//! let client = RedditBuilder::new()
//!     .user_agent("linux:roux:v1.3.8 (by /u/blars_tacoman)")
//!     .build()
//!     .await?;
//! let subreddits = Subreddits::new(&reddit)
//!     .search("cats", Some(50), None)
//!     .await?;
//! ```
//!
//! # Submit A Text Post
//! ```no_run
//! use roux::Reddit;
//! # use tokio_test;
//!
//! # tokio_test::block_on(async {
//! let client = Reddit::new("USER_AGENT", "CLIENT_ID", "CLIENT_SECRET")
//!     .username("USERNAME")
//!     .password("PASSWORD")
//!     .login()
//!     .await;
//!
//! let me = client.unwrap();
//!
//! me.submit_text("TEXT_TITLE", "TEXT_BODY", "SUBREDDIT");
//! # })
//! ```
//!
//! # Submit A Link Post
//! ```no_run
//! use roux::Reddit;
//! # use tokio_test;
//!
//! # tokio_test::block_on(async {
//! let client = Reddit::new("USER_AGENT", "CLIENT_ID", "CLIENT_SECRET")
//!     .username("USERNAME")
//!     .password("PASSWORD")
//!     .login()
//!     .await;
//!
//! let me = client.unwrap();
//!
//! me.submit_link("LINK_TITLE", "LINK", "SUBREDDIT");
//! # })
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
