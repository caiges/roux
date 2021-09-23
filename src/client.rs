use crate::config::Config;
use crate::util::{url, RouxError};
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION, USER_AGENT},
    Client as ReqwestClient, ClientBuilder as ReqwestClientBuilder,
};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct AuthData {
    pub access_token: String,
}

/// A HTTP client for making requests to the Reddit API.
pub struct Client {
    access_token: String,
    client: ReqwestClient,
}

/// A builder type for creating configured clients.
pub struct ClientBuilder {
    config: Config,
}

impl ClientBuilder {
    /// Create a new client builder.
    pub fn new() -> Self {
        Self {
            config: Config::new("", "", ""),
        }
    }

    /// Set the user agent.
    pub fn user_agent(mut self, user_agent: &str) -> Self {
        self.config.user_agent = user_agent.to_owned();
        self
    }

    /// Set the client id.
    pub fn client_id(mut self, client_id: &str) -> Self {
        self.config.client_id = client_id.to_owned();
        self
    }

    /// Set the client secret.
    pub fn client_secret(mut self, client_secret: &str) -> Self {
        self.config.client_secret = client_secret.to_owned();
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
    pub async fn build(self) -> Result<Client, RouxError> {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, self.config.user_agent[..].parse().unwrap());
        let client = ReqwestClientBuilder::new()
            .default_headers(headers)
            .build()
            .unwrap();

        let url = &url::build_url("api/v1/access_token")[..];
        let form = [("grant_type", "client_credentials")];

        let request = client
            .post(url)
            .header(USER_AGENT, &self.config.user_agent[..])
            .basic_auth(&self.config.client_id, Some(&self.config.client_secret))
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

            let subreddit_client = ReqwestClientBuilder::new()
                .default_headers(headers)
                .build()
                .unwrap();

            Ok(Client {
                access_token: auth_data.access_token,
                client: subreddit_client,
            })
        } else {
            Err(RouxError::Status(response))
        }
    }
}
