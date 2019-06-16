use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use crate::clients::api::ApiClient;
use crate::clients::{get_root_url, GitHubClientOptions};

pub struct RepositoriesClient {
    pub options: GitHubClientOptions,
}

impl ApiClient for RepositoriesClient {}

impl RepositoriesClient {
    pub(crate) fn for_org_name(&self, org_name: &str, type_name: Option<String>, sort: Option<String>, direction: Option<String>) -> Vec<Repository> {
        let url =
            format!("{root}/orgs/{org_name}/repos?type={type_name}&sort={sort}&direction={direction}",
                    root = get_root_url(),
                    org_name = org_name,
                    type_name = type_name.unwrap_or("all".to_string()),
                    sort = sort.unwrap_or("created".to_string()),
                    direction = direction.unwrap_or("desc".to_string())
            );

        self.get_many::<Repository>(&self.options, url.as_str(), None, None)
    }
}

#[derive(Deserialize)]
pub struct Repository {
    pub id: u64,
    pub node_id: Option<String>,
    pub name: Option<String>,
    pub full_name: Option<String>,
    pub owner: RepositoryOwner,
    pub private: bool,
    pub html_url: Option<String>,
    pub description: Option<String>,
    pub fork: bool,
    pub url: Option<String>,
    pub archive_url: Option<String>,
    pub assignees_url: Option<String>,
    pub blobs_url: Option<String>,
    pub branches_url: Option<String>,
    pub collaborators_url: Option<String>,
    pub comments_url: Option<String>,
    pub commits_url: Option<String>,
    pub compare_url: Option<String>,
    pub contents_url: Option<String>,
    pub contributors_url: Option<String>,
    pub deployments_url: Option<String>,
    pub downloads_url: Option<String>,
    pub events_url: Option<String>,
    pub forks_url: Option<String>,
    pub git_commits_url: Option<String>,
    pub git_refs_url: Option<String>,
    pub git_tags_url: Option<String>,
    pub git_url: Option<String>,
    pub issue_comment_url: Option<String>,
    pub issue_events_url: Option<String>,
    pub issues_url: Option<String>,
    pub keys_url: Option<String>,
    pub labels_url: Option<String>,
    pub languages_url: Option<String>,
    pub merges_url: Option<String>,
    pub milestones_url: Option<String>,
    pub notifications_url: Option<String>,
    pub pulls_url: Option<String>,
    pub releases_url: Option<String>,
    pub ssh_url: Option<String>,
    pub stargazers_url: Option<String>,
    pub statuses_url: Option<String>,
    pub subscribers_url: Option<String>,
    pub subscription_url: Option<String>,
    pub tags_url: Option<String>,
    pub teams_url: Option<String>,
    pub trees_url: Option<String>,
    pub clone_url: Option<String>,
    pub mirror_url: Option<String>,
    pub hooks_url: Option<String>,
    pub svn_url: Option<String>,
    pub homepage: Option<String>,
    pub language: Option<String>,
    pub forks_count: Option<usize>,
    pub stargazers_count: Option<usize>,
    pub watchers_count: Option<usize>,
    pub size: Option<usize>,
    pub default_branch: Option<String>,
    pub open_issues_count: Option<usize>,
    pub topics: HashSet<String>,
    pub has_issues: bool,
    pub has_projects: bool,
    pub has_wiki: bool,
    pub has_pages: bool,
    pub has_downloads: bool,
    pub archived: bool,
    pub disabled: bool,
    pub pushed_at: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub permissions: HashMap<String, bool>,
    pub subscribers_count: usize,
    pub network_count: usize,
    pub license: RepositoryLicense,
}

#[derive(Deserialize)]
pub struct RepositorySummary {
    pub id: Option<String>,
    pub node_id: Option<String>,
    pub name: Option<String>,
    pub full_name: Option<String>,
    pub owner: RepositoryOwner,
    pub private: bool,
    pub html_url: Option<String>,
    pub description: Option<String>,
    pub fork: bool,
    pub url: Option<String>,
    pub archive_url: Option<String>,
    pub assignees_url: Option<String>,
    pub blobs_url: Option<String>,
    pub branches_url: Option<String>,
    pub collaborators_url: Option<String>,
    pub comments_url: Option<String>,
    pub commits_url: Option<String>,
    pub compare_url: Option<String>,
    pub contents_url: Option<String>,
    pub contributors_url: Option<String>,
    pub deployments_url: Option<String>,
    pub downloads_url: Option<String>,
    pub events_url: Option<String>,
    pub forks_url: Option<String>,
    pub git_commits_url: Option<String>,
    pub git_refs_url: Option<String>,
    pub git_tags_url: Option<String>,
    pub git_url: Option<String>,
    pub issue_comment_url: Option<String>,
    pub issue_events_url: Option<String>,
    pub issues_url: Option<String>,
    pub keys_url: Option<String>,
    pub labels_url: Option<String>,
    pub languages_url: Option<String>,
    pub merges_url: Option<String>,
    pub milestones_url: Option<String>,
    pub notifications_url: Option<String>,
    pub pulls_url: Option<String>,
    pub releases_url: Option<String>,
    pub ssh_url: Option<String>,
    pub stargazers_url: Option<String>,
    pub statuses_url: Option<String>,
    pub subscribers_url: Option<String>,
    pub subscription_url: Option<String>,
    pub tags_url: Option<String>,
    pub teams_url: Option<String>,
    pub trees_url: Option<String>,
}

#[derive(Deserialize)]
pub struct RepositoryOwner {
    pub login: String,
    pub id: String,
    pub node_id: Option<String>,
    pub avatar_url: Option<String>,
    pub gravatar_id: Option<String>,
    pub url: Option<String>,
    pub html_url: Option<String>,
    pub followers_url: Option<String>,
    pub following_url: Option<String>,
    pub gists_url: Option<String>,
    pub starred_url: Option<String>,
    pub subscriptions_url: Option<String>,
    pub organizations_url: Option<String>,
    pub repos_url: Option<String>,
    pub events_url: Option<String>,
    pub received_events_url: Option<String>,
    #[serde(rename = "type")]
    pub type_name: Option<String>,
    pub site_admin: bool,
}

#[derive(Deserialize)]
pub struct RepositoryLicense {
    pub key: Option<String>,
    pub name: Option<String>,
    pub spdx_id: Option<String>,
    pub url: Option<String>,
    pub node_id: Option<String>,
}