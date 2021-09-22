/// Config can be used to configure clients.
pub struct Config {
    /// user agent for requests.
    pub user_agent: String,
    /// client id used when authenticating.
    pub client_id: String,
    /// client secret used when authenticationg.
    pub client_secret: String,
    /// username used when authenticating requests requiring a user context.
    pub username: Option<String>,
    /// password used when authenticating requests requiring a user context.
    pub password: Option<String>,
    /// access token used when authenticating requests.
    pub access_token: Option<String>,
}

impl Config {
    /// Create a new Config instance.
    pub fn new(user_agent: &str, client_id: &str, client_secret: &str) -> Config {
        Config {
            user_agent: user_agent.to_owned(),
            client_id: client_id.to_owned(),
            client_secret: client_secret.to_owned(),
            username: None,
            password: None,
            access_token: None,
        }
    }
}
