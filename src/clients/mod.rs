use std::time::Duration;
use reqwest::{Client};
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT, AUTHORIZATION};
use serde::{Deserialize};
use crate::clients::organizations::OrganizationsClient;
use crate::clients::repositories::RepositoriesClient;
use std::alloc::rust_oom;

pub mod organizations;
pub mod repositories;

mod api;

fn get_root_url() -> &'static str {
    "https://api.github.com"
}

pub struct GitHubClient {
    pub organizations: OrganizationsClient,
    pub repositories: RepositoriesClient,
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
    /// use roctokit::clients::GitHubClientBuilder;
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
    /// use roctokit::clients::GitHubClientBuilder;
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
    /// use roctokit::clients::GitHubClientBuilder;
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
    /// use roctokit::clients::GitHubClientBuilder;
    /// let builder =
    ///     GitHubClientBuilder::new()
    ///         .with_oauth_token("token goes here");
    /// ```
    pub fn with_oauth_token(&mut self, oauth_token: &str) -> &mut GitHubClientBuilder {
        self.token = Some(oauth_token.to_string());
        self
    }

    /// Build a `GitHubClient` to begin interrogating the GitHub API```
    pub fn build(&self) -> GitHubClient {
        let client = self.build_client();
        let root_document = client.get_root_document();

        GitHubClient {
            repositories: RepositoriesClient {
                client,
                base_url: root_document.repository_url.unwrap(),
            },
            organizations: OrganizationsClient {
                client,
                base_url: root_document.organization_url.unwrap(),
            },
        }
    }

    fn build_client(&self) -> Client {
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
}

pub trait RootClientEx {
    fn get_root_document(&self) -> RootDocument;
}

impl RootClientEx for Client {
    fn get_root_document(&self) -> RootDocument {
        let result = self
            .get(get_root_url())
            .send();
        let mut response = result.unwrap();
        if !response.status().is_success() {
            panic!("unable to read the root resource from {base_url}: {error_code}", base_url = get_root_url(), error_code = response.status());
        }
        response
            .json::<RootDocument>()
            .unwrap()
    }
}

#[derive(Deserialize)]
pub struct RootDocument {
    pub current_user_url: Option<String>,
    pub current_user_authorizations_html_url: Option<String>,
    pub authorizations_url: Option<String>,
    pub code_search_url: Option<String>,
    pub commit_search_url: Option<String>,
    pub emails_url: Option<String>,
    pub emojis_url: Option<String>,
    pub events_url: Option<String>,
    pub feeds_url: Option<String>,
    pub followers_url: Option<String>,
    pub following_url: Option<String>,
    pub gists_url: Option<String>,
    pub hub_url: Option<String>,
    pub issue_search_url: Option<String>,
    pub issues_url: Option<String>,
    pub keys_url: Option<String>,
    pub notifications_url: Option<String>,
    pub organization_repositories_url: Option<String>,
    pub organization_url: Option<String>,
    pub public_gists_url: Option<String>,
    pub rate_limit_url: Option<String>,
    pub repository_url: Option<String>,
    pub repository_search_url: Option<String>,
    pub current_user_repositories_url: Option<String>,
    pub starred_url: Option<String>,
    pub starred_gists_url: Option<String>,
    pub team_url: Option<String>,
    pub user_url: Option<String>,
    pub user_organizations_url: Option<String>,
    pub user_repositories_url: Option<String>,
    pub user_search_url: Option<String>
}

impl RootDocument {
    pub fn new() -> RootDocument {
        RootDocument {
            current_user_url: None,
            current_user_authorizations_html_url: None,
            authorizations_url: None,
            code_search_url: None,
            commit_search_url: None,
            emails_url: None,
            emojis_url: None,
            events_url: None,
            feeds_url: None,
            followers_url: None,
            following_url: None,
            gists_url: None,
            hub_url: None,
            issue_search_url: None,
            issues_url: None,
            keys_url: None,
            notifications_url: None,
            organization_repositories_url: None,
            organization_url: None,
            public_gists_url: None,
            rate_limit_url: None,
            repository_url: None,
            repository_search_url: None,
            current_user_repositories_url: None,
            starred_url: None,
            starred_gists_url: None,
            team_url: None,
            user_url: None,
            user_organizations_url: None,
            user_repositories_url: None,
            user_search_url: None
        }
    }
}