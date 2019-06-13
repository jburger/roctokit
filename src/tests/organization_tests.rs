use crate::client::github_client::{GitHubClient, GitHubClientBuilder};
use std::env;
use reqwest::Client;

const EXPECTED_ORG_NAME: &str = "silentec";

#[test]
fn it_finds_organizations_token_auth() {
    let mut client = get_token_auth_client();
    let organization_result = client.organizations.get_by_name(EXPECTED_ORG_NAME);
    assert_eq!(organization_result.to_string(), "org: silentec 24204496  https://api.github.com/orgs/silentec");
}

#[test]
fn matches_raw() {
    let expected =
        Client::new()
            .get("https://api.github.com/orgs/silentec")
            .bearer_auth(env::var("github_token").unwrap())
            .send().unwrap()
            .text().unwrap();
    let mut client = get_token_auth_client();
    let organization_result = client.organizations.get_by_name(EXPECTED_ORG_NAME);
    let actual  = serde_json::to_string(&organization_result).unwrap();
    assert_eq!(expected, actual);
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