use crate::clients::{*, organizations::*};
use std::env;

const EXPECTED_ORG_NAME: &str = "github";
const USER_AGENT: &str = "roctokit";

#[test]
fn it_finds_organizations_token_auth() {
    let organization_result = get_test_org();
    assert_eq!(organization_result.to_string(), "org: GitHub 9919 How people build software. https://api.github.com/orgs/github");
}

#[test]
fn it_retrieves_all_organizations_for_a_user() {
    let client = get_token_auth_client();
    let users_orgs = client.organizations.get_all_for_user();
    assert!(users_orgs.len() > 0);
}

#[test]
fn it_retrieves_first_100_organizations() {
    let client = get_token_auth_client();
    let orgs = client.organizations.top();
    assert_eq!(orgs.len(), 100);
}

#[test]
fn it_retrieves_a_number_of_organizations() {
    let client = get_token_auth_client();
    let orgs = client.organizations.some(1, 42);
    assert_eq!(orgs.len(), 42);
}

fn get_test_org() -> Organization {
    let mut client = get_token_auth_client();
    let organization_result = client.organizations.get_by_name(EXPECTED_ORG_NAME);
    organization_result
}

fn get_token_auth_client() -> GitHubClient {
    let mut builder = GitHubClientBuilder::new();
    builder.for_user_agent(USER_AGENT);

    match env::var("github_token") {
        Ok(token) =>
            builder
            .with_oauth_token(token.as_str())
            .build(),
        Err(_) => builder.build()
    }
}