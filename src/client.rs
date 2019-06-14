use std::time::Duration;
use reqwest::{Client};
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT, AUTHORIZATION};
use crate::model::Organization;
use crate::model::RootDocument;
const DEFAULT_BASE_URL: &str = "https://api.github.com";

pub struct GitHubClient {
    pub organizations: OrganizationsRepository,
}

pub struct GitHubClientBuilder {
    timeout_in_secs: Option<u64>,
    user_agent_string: String,
    token: Option<String>,
}

impl GitHubClientBuilder {
    /// Creates a factory that configures a `GitHubClient`.
    /// # Examples
    ///
    ///  ```
    /// use roctokit::client::{GitHubClientBuilder};
    /// let builder = GitHubClientBuilder::new();
    /// ```
    pub fn new() -> GitHubClientBuilder {
        self::GitHubClientBuilder {
            timeout_in_secs: None,
            user_agent_string: String::new(),
            token: None
        }
    }

    /// Sets the timeout for HTTP requests.
    /// # Examples
    /// ```
    /// use roctokit::client::{GitHubClientBuilder};
    /// let builder =
    ///     GitHubClientBuilder::new()
    ///         .with_timeout(20); // 20 seconds
    /// ```
    pub fn with_timeout(&mut self, timeout_in_seconds: u64) -> &mut GitHubClientBuilder {
        self.timeout_in_secs = Some(timeout_in_seconds);
        self
    }

    /// The github API allows for callers to identify themselves using a user agent string, this method sets the agent for all HTTP calls.
    /// # Examples
    /// ```
    /// use roctokit::client::{GitHubClientBuilder};
    /// let builder =
    ///     GitHubClientBuilder::new()
    ///         .for_user_agent("roctokit");
    /// ```
    pub fn for_user_agent(&mut self, user_agent_string: &str) -> &mut GitHubClientBuilder {
        self.user_agent_string = user_agent_string.to_string();
        self
    }

    /// Authenticate using an GitHub personal access OAUTH token
    /// # Examples
    /// ```
    /// use roctokit::client::{GitHubClientBuilder};
    /// let builder =
    ///     GitHubClientBuilder::new()
    ///         .with_oauth_token("token goes here");
    /// ```
    pub fn with_oauth_token(&mut self, oauth_token: &str) -> &mut GitHubClientBuilder {
        self.token = Some(oauth_token.to_string());
        self
    }

    /// Build a `GitHubClient` to begin interrogating the GitHub API.
    /// # Examples
    /// ```
    /// use roctokit::client::{GitHubClientBuilder};
    /// let mut client =
    ///     GitHubClientBuilder::new()
    ///         .for_user_agent("roctokit")
    ///         .build();
    /// ```
    pub fn build(&self) -> GitHubClient {
        let client = self.get_client();
        let root_document = GitHubClientBuilder::get_root_document(&client);

        GitHubClient {
            organizations: OrganizationsRepository {
                client,
                base_url: root_document.organization_url.unwrap(),
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
            USER_AGENT,
            HeaderValue::from_str(self.user_agent_string.as_str()).unwrap()
        );

        match &self.token {
            None => {}
            Some(token) => {
                header_map.append(
                    AUTHORIZATION,
                    HeaderValue::from_str(format!("token {}", token).as_str()).unwrap());
            }
        }

        builder
            .default_headers(header_map)
            .build()
            .unwrap()
    }

    fn get_root_document(client: &Client) -> RootDocument {
        let result = client
            .get(DEFAULT_BASE_URL)
            .send();
        let mut response = result.unwrap();
        if !response.status().is_success() {
            panic!("unable to read the root resource from {base_url}: {error_code}", base_url = DEFAULT_BASE_URL, error_code = response.status());
        }
        response
            .json::<RootDocument>()
            .unwrap()
    }
}

pub struct OrganizationsRepository {
    pub client: Client,
    pub base_url: String,
}

impl OrganizationsRepository {
    /// Get an organization by name.
    ///
    /// # Examples
    /// ```
    /// use roctokit::client::GitHubClientBuilder;
    /// let mut client = GitHubClientBuilder::new()
    ///     .for_user_agent("roctokit")
    ///     .build();
    /// let github = client.organizations.get_by_name("github");
    /// ```
    pub fn get_by_name(&mut self, name: &str) -> Organization {
        let url = self.base_url.as_str().replace("{org}", name);
        let result = self.client.get(url.as_str()).send();

        match result {
            Ok(mut response) => {
                let x = response.json::<Organization>();
                match x {
                    Ok(org) => org,
                    Err(error) => {
                        panic!(format!("{}", error));
                    }
                }
            },
            Err(error) => {
               panic!(format!("{}", error));
            }
        }
    }
}