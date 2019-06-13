use std::time::Duration;
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::header;
use crate::client::organizations::{ Organizations };
use crate::model::{RootDocument};

const DEFAULT_BASE_URL: &str = "https://api.github.com";

pub struct GitHubClient {
    pub organizations: Organizations,
}

pub struct GitHubClientBuilder {
    timeout_in_secs: Option<u64>,
    user_agent_string: String,
    username: Option<String>,
    password: Option<String>,
    token: Option<String>,
}

impl GitHubClientBuilder {
    pub fn new() -> GitHubClientBuilder {
        self::GitHubClientBuilder {
            timeout_in_secs: None,
            user_agent_string: String::new(),
            username: None,
            password: None,
            token: None
        }
    }

    pub fn with_timeout(&mut self, timeout_in_seconds: u64) -> &mut GitHubClientBuilder {
        self.timeout_in_secs = Some(timeout_in_seconds);
        self
    }

    pub fn for_user_agent(&mut self, user_agent_string: &str) -> &mut GitHubClientBuilder {
        self.user_agent_string = user_agent_string.to_string();
        self
    }

    pub fn with_bearer_token(&mut self, bearer_token: &str) -> &mut GitHubClientBuilder {
        self.token = Some(bearer_token.to_string());
        self
    }

    pub fn with_basic_auth(&mut self, username: &str, password: &str) -> & mut GitHubClientBuilder {
        self.username = Some(username.to_string());
        self.password = Some(password.to_string());
        self
    }

    pub fn build(&self) -> GitHubClient {
        let client = self.get_client();
        let root_document = GitHubClientBuilder::get_root_document(&client);

        GitHubClient {
            organizations: Organizations {
                client: client,
                base_url: root_document.organization_url.unwrap()
            },

        }
    }

    fn get_client(&self) -> Client {
        reqwest::ClientBuilder::new()
            .timeout(
                Duration::from_secs(self.timeout_in_secs.unwrap_or(10)))
            .default_headers(GitHubClientBuilder::get_default_headers(self))
            .build()
            .unwrap()
    }

    fn get_default_headers(&self) -> HeaderMap<HeaderValue> {
        let mut header_map = HeaderMap::new();

        header_map.append(
            header::USER_AGENT,
            HeaderValue::from_str(self.user_agent_string.as_str()).unwrap()
        );

        match &self.username {
            None => match &self.token {
                None => header_map,
                Some(token) => {
                    let bearer_header_value = format!("Bearer {}", token);
                    header_map.append(
                        header::AUTHORIZATION,
                        HeaderValue::from_str(bearer_header_value.as_str()).unwrap());
                    header_map
                }
            },
            Some(username) => {
                let auth_header_value = format!("{}:{}", username, &self.password.as_ref().unwrap());
                header_map.append(
                    header::AUTHORIZATION,
                    HeaderValue::from_str(auth_header_value.as_str()).unwrap());
                header_map
            }
        }
    }

    pub fn get_root_document(client: &Client) -> RootDocument {
        client
            .get(DEFAULT_BASE_URL)
            .send()
            .unwrap()
            .json::<RootDocument>()
            .unwrap()
    }
}