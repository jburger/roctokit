use crate::client::github_client::{GitHubClient, GitHubClientBuilder};
use std::env;

const EXPECTED_ORG_NAME: &str = "silentec";

#[test]
fn it_finds_organizations_token_auth() {
    let client = get_token_auth_client();
    assert_find_organization(client);
}

fn assert_find_organization(client: GitHubClient) {
    let organization_result = client.organizations.find(EXPECTED_ORG_NAME);
    assert!(organization_result.is_ok());
    assert_eq!(organization_result.unwrap().name, EXPECTED_ORG_NAME);
}

fn get_token_auth_client() -> GitHubClient {
    let mut builder = GitHubClientBuilder::new();
    let token = env::var("github_token");
    assert!(token.is_ok());

    builder
        .for_user_agent(EXPECTED_ORG_NAME)
        .with_oauth_token(token.unwrap_or_default().as_str())
        .build()
}