use std::string::ToString;
use reqwest::Client;
use serde::{Serialize, Deserialize};

use crate::clients::{api::{ApiClient}, get_root_url, RootClientEx};

pub struct OrganizationsClient {
    pub client: Client,
    pub base_url: String,
}

impl ApiClient for OrganizationsClient {
    fn get_client(&self) -> &Client {
        &self.client
    }
}

impl OrganizationsClient {
    /// Get an organization by name.
    ///
    /// # Examples
    /// ```
    /// use std::env;
    /// use roctokit::clients::GitHubClientBuilder;
    /// let mut client = GitHubClientBuilder::new()
    ///     .for_user_agent("roctokit")
    ///     .with_oauth_token(env::var("github_token").unwrap().as_str())
    ///     .build();
    ///
    /// let github = client.organizations.get_by_name("github");
    /// ```
    pub fn get_by_name(&mut self, name: &str) -> Organization {
        let url = self.base_url.replace("{org}", name);
        self.get(url.as_str())
    }

    /// Get all organizations for the authenticated user. Must be authenticated.
    ///
    /// # Examples
    /// ```
    /// use std::env;
    /// use roctokit::clients::GitHubClientBuilder;
    /// let mut client = GitHubClientBuilder::new()
    ///     .for_user_agent("roctokit")
    ///     .with_oauth_token(env::var("github_token").unwrap().as_str())
    ///     .build();
    ///
    /// let github = client.organizations.get_all_for_user();
    /// ```
    /// # Panics
    /// - When the caller is unauthenticated
    pub fn get_all_for_user(&self) -> Vec<OrganizationSummary> {
        let url = self.client.get_root_document().user_organizations_url.unwrap();
        self.get::<Vec<OrganizationSummary>>(url.as_str())
    }

    /// Get all organizations, this can take a while.
    /// todo: asyncrony
    pub fn all(&self) -> Vec<OrganizationSummary> {
        self.get_many::<OrganizationSummary>(format!("{}/organizations", get_root_url()).as_str(), Some(1), Some(std::usize::MAX))
    }

    /// Get a window of results starting at `since` for a maximum of `limit` items.
    pub fn some(&self, since: usize, limit: usize) -> Vec<OrganizationSummary> {
        self.get_many::<OrganizationSummary>(format!("{}/organizations", get_root_url()).as_str(), Some(since), Some(limit))
    }

    /// Get the top 100 organization results.
    pub fn top(&self) -> Vec<OrganizationSummary> {
        self.get_many::<OrganizationSummary>(format!("{}/organizations", get_root_url()).as_str(), Some(1), Some(100))
    }
}

#[derive(Serialize, Deserialize)]
pub struct Organization {
    pub login: String,
    pub id: u64,
    pub node_id: String,
    pub url: String,
    pub repos_url: String,
    pub events_url: String,
    pub hooks_url: String,
    pub issues_url: String,
    pub members_url: String,
    pub public_members_url: String,
    pub avatar_url: String,
    pub description: String,
    pub name: String,
    pub company: Option<String>,
    pub blog: Option<String>,
    pub location: Option<String>,
    pub email: Option<String>,
    pub is_verified: bool,
    pub has_organization_projects: bool,
    pub has_repository_projects: bool,
    pub public_repos: u64,
    pub public_gists: u64,
    pub followers: Option<u64>,
    pub following: Option<u64>,
    pub html_url: String,
    pub created_at: String,
    pub updated_at: String,
    #[serde(rename = "type")]
    pub type_name: String,
    pub total_private_repos: Option<u64>,
    pub owned_private_repos: Option<u64>,
    pub private_gists: Option<u64>,
    pub disk_usage: Option<u64>,
    pub collaborators: Option<u64>,
    pub billing_email: Option<String>,
    pub default_repository_permission: Option<String>,
    pub members_can_create_repositories: Option<bool>,
    pub two_factor_requirement_enabled: Option<bool>,
    pub plan: Option<GitHubPlanInfo>
}

#[derive(Deserialize)]
pub struct OrganizationSummary {
    pub login: String,
    pub id: u64,
    pub node_id: String,
    pub url: String,
    pub repos_url: Option<String>,
    pub events_url: Option<String>,
    pub hooks_url: Option<String>,
    pub issues_url: Option<String>,
    pub members_url: Option<String>,
    pub public_members_url: Option<String>,
    pub avatar_url: Option<String>,
    pub description: Option<String>,
}

impl ToString for Organization {
    fn to_string(&self) -> String {
        format!("org: {name} {id} {description} {url}",
                name = self.name,
                id = self.id,
                description = self.description,
                url = self.url)
    }
}

#[derive(Serialize, Deserialize)]
pub struct GitHubPlanInfo {
    pub name: Option<String>,
    pub space: Option<u64>,
    pub private_repos: Option<u64>,
    pub filled_seats: Option<u64>,
    pub seats: Option<u64>
}