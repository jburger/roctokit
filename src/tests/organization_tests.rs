use crate::client::{GitHubClient, GitHubClientBuilder};
use crate::model::Organization;
use std::env;
use reqwest::Client;

const EXPECTED_ORG_NAME: &str = "silentec";

#[test]
fn it_finds_organizations_token_auth() {
    let organization_result = get_test_org();
    assert_eq!(organization_result.to_string(), "org: silentec 24204496  https://api.github.com/orgs/silentec");
}

fn get_test_org() -> Organization {
    let mut client = get_token_auth_client();
    let organization_result = client.organizations.get_by_name(EXPECTED_ORG_NAME);
    organization_result
}

#[test]
fn org_result_from_roctokit_matches_http_call_result() {
    let http_call_result = get_org_via_http_call();
    let github_client_result = get_org_via_github_client();
    assert_eq!(http_call_result, github_client_result);

    fn get_org_via_http_call() -> String {
        Client::new()
            .get("https://api.github.com/orgs/silentec")
            .bearer_auth(env::var("github_token").unwrap())
            .send().unwrap()
            .text().unwrap()
    }

    fn get_org_via_github_client() -> String {
        let organization_result = get_test_org();
        let actual = serde_json::to_string(&organization_result).unwrap();
        actual
    }
}

fn get_token_auth_client() -> GitHubClient {
    let mut builder = GitHubClientBuilder::new();
    let token = env::var("github_token");
    assert!(token.is_ok(), "Please make sure you have a 'github_token' environment variable set");

    builder
        .for_user_agent(EXPECTED_ORG_NAME)
        .with_oauth_token(token.unwrap_or_default().as_str())
        .build()
}