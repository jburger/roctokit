use serde::{Serialize, Deserialize};

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