use crate::client::{GitHubClient, GitHubClientBuilder};
use crate::model::Organization;
use std::env;

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

fn get_token_auth_client() -> GitHubClient {
    let mut builder = GitHubClientBuilder::new();
    builder.for_user_agent(EXPECTED_ORG_NAME);

    match env::var("github_token") {
        Ok(token) =>
            builder
            .with_oauth_token(token.as_str())
            .build(),
        Err(_) => builder.build()
    }
}