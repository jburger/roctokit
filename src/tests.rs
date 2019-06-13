use crate::client::github_client::{GitHubClient, GitHubClientBuilder};
use std::env;

const EXPECTED_ORG_NAME: &str = "silentec";

#[test]
fn it_can_be_configured_with_token_auth() {
    get_token_auth_client();
}

#[test]
fn it_can_be_configured_with_timeout() {
    get_unauthenticated_client_with_timeout();
}

#[test]
fn it_finds_organizations_anonymously() {
    let client = get_unauthenticated_client();
    assert_find_organization(client);
}

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

fn get_unauthenticated_client() -> GitHubClient {
    let mut builder = GitHubClientBuilder::new();
    builder
        .for_user_agent(EXPECTED_ORG_NAME)
        .build()
}

fn get_token_auth_client() -> GitHubClient {
    let mut builder = GitHubClientBuilder::new();
    let token = env::var("github_token");

    builder
        .for_user_agent(EXPECTED_ORG_NAME)
        .with_oauth_token(token.unwrap().as_str())
        .build()
}

fn get_unauthenticated_client_with_timeout() -> GitHubClient {
    let mut builder = GitHubClientBuilder::new();
    builder
        .with_timeout(60)
        .for_user_agent(EXPECTED_ORG_NAME)
        .build()
}