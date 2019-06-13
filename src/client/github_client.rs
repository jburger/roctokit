use std::time::Duration;
use reqwest::{Client};
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
    token: Option<String>,
}

impl GitHubClientBuilder {
    pub fn new() -> GitHubClientBuilder {
        self::GitHubClientBuilder {
            timeout_in_secs: None,
            user_agent_string: String::new(),
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

    pub fn with_oauth_token(&mut self, oauth_token: &str) -> &mut GitHubClientBuilder {
        self.token = Some(oauth_token.to_string());
        self
    }

    pub fn build(&self) -> GitHubClient {
        let client = self.get_client();
        let root_document = GitHubClientBuilder::get_root_document(&client);

        GitHubClient {
            organizations: Organizations {
                client,
                base_url: root_document.organization_url.unwrap()
            },
        }
    }

    fn get_client(&self) -> Client {
        let builder =
            reqwest::ClientBuilder::new()
                .timeout(
                    Duration::from_secs(self.timeout_in_secs.unwrap_or(10)));

        let mut header_map = HeaderMap::new();
        header_map.append(
            header::USER_AGENT,
            HeaderValue::from_str(self.user_agent_string.as_str()).unwrap()
        );

        match &self.token {
            None => {}
            Some(token) => {
                header_map.append(
                    header::AUTHORIZATION,
                    HeaderValue::from_str(format!("token {}", token).as_str()).unwrap());
            }
        }

        builder
            .default_headers(header_map)
            .build()
            .unwrap()
    }

    pub fn get_root_document(client: &Client) -> RootDocument {
        let result = client
            .get(DEFAULT_BASE_URL)
            .send();
        let mut response = result.unwrap();
        if !response.status().is_success() {
            panic!("unable to read the root resource from {base_url}: {error_code}", base_url = DEFAULT_BASE_URL, error_code = response.status());
        }
        response
            .json::<RootDocument>()
            .unwrap_or(RootDocument::new())
    }
}