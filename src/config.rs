pub const DEFAULT_URL: &str = "https://reddit.com";
pub const DEFAULT_AUTHENTICATED_URL: &str = "https://oauth.reddit.com";

/// Config can be used to configure clients.
pub struct Config {
    /// url for requests
    pub url: String,
    /// user agent for requests.
    pub user_agent: String,
    /// client id used when authenticating.
    pub client_id: Option<String>,
    /// client secret used when authenticationg.
    pub client_secret: Option<String>,
    /// username used when authenticating requests requiring a user context.
    pub username: Option<String>,
    /// password used when authenticating requests requiring a user context.
    pub password: Option<String>,
    /// access token used when authenticating requests.
    pub access_token: Option<String>,
}

impl Config {
    /// Create a new Config instance.
    pub fn new() -> Self {
        Self {
            url: DEFAULT_URL.to_owned(),
            user_agent: "roux".to_string(),
            client_id: None,
            client_secret: None,
            username: None,
            password: None,
            access_token: None,
        }
    }
}
