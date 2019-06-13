use serde::Deserialize;

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

#[derive(Deserialize)]
pub struct Organization {
    pub name: String
}