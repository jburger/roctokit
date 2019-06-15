use std::time::Duration;
use reqwest::{Client};
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT, AUTHORIZATION};
use serde::{Deserialize};
use crate::clients::organizations::OrganizationsClient;

fn get_root_url() -> &'static str {
    "https://api.github.com"
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

pub mod organizations;

trait ApiClient {
    fn get_client(&self) -> &Client;

    fn get<T>(&self, route: &str) -> T where for<'de> T: serde::Deserialize<'de> {
        let result = self.get_client().get(route).send();
        match result {
            Ok(mut response) => {
                if !response.status().is_success() {
                    panic!(format!("Unable to access resource: {}", response.status()));
                }
                let deserialized = response.json::<T>();
                match deserialized {
                    Ok(resource) => resource,
                    Err(error) => {
                        panic!(format!("Unable to deserialize response body: {}", error));
                    }
                }
            },
            Err(error) => {
                panic!(format!("Unable to complete HTTP request: {}", error));
            }
        }
    }

    fn get_many<T>(&self, route: &str, since: Option<u64>, limit: Option<u64>) -> Vec<T> where for<'de> T: serde::Deserialize<'de> {
        use regex::Regex;

        let limit = limit.unwrap_or(1000);
        let client = self.get_client();

        let mut response= client.get(format!("{}?since={}",route, since.unwrap_or(1)).as_str()).send().unwrap();
        let mut resources = Vec::<T>::new();
        let mut proceed: bool = true;

        while proceed {
            let deserialized = response.json::<Vec<T>>();

            match deserialized {
                Ok(mut result) => {
                    resources.append(&mut result);
                    if resources.len() > limit as usize {
                        resources.truncate(limit as usize);
                        break;
                    }
                },
                Err(error) => {
                    panic!(format!("Unable to deserialize response body: {}", error));
                }
            }

            proceed = match response.headers().get("Link") {
                None => {
                   false
                },
                Some(link_header) => {
                    let x: Result<&str, _> = link_header.to_str();
                    match x {
                        Err(_) => {
                            false
                        },
                        Ok(header) => {
                            let links: Vec<&str> = header.split(";").collect();
                            if links.len() > 0 {
                                let rgx = Regex::new(r"[<>]").unwrap();
                                let url = rgx.replace_all(links[0], "").to_string();
                                response = client.get(url.as_str()).send().unwrap();
                                true
                            } else {
                                false
                            }
                        },
                    }
                },
            }
        }
        resources
    }
}

pub struct GitHubClient {
    pub organizations: OrganizationsClient,
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