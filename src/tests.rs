use crate::client::github_client::{GitHubClient, GitHubClientBuilder};

const EXPECTED_ORG_NAME: &str = "silentec";

#[test]
fn it_can_be_configured_with_basic_auth() {
    get_basic_auth_client();
}

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

fn get_basic_auth_client() -> GitHubClient {
    let mut builder = GitHubClientBuilder::new();
    builder
        .for_user_agent(EXPECTED_ORG_NAME)
        .with_basic_auth("username", "password")
        .build()
}


fn get_token_auth_client() -> GitHubClient {
    let mut builder = GitHubClientBuilder::new();
    builder
        .for_user_agent(EXPECTED_ORG_NAME)
        .with_bearer_token("token goes here")
        .build()
}


fn get_unauthenticated_client_with_timeout() -> GitHubClient {
    let mut builder = GitHubClientBuilder::new();
    builder
        .with_timeout(60)
        .for_user_agent(EXPECTED_ORG_NAME)
        .build()
}