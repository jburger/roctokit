use crate::clients::{GitHubClient, GitHubClientBuilder};
use std::env;

const EXPECTED_ORG_NAME: &str = "silentec";
const USER_AGENT: &str = "roctokit";

mod organization_tests;
mod repository_tests;

fn get_token_auth_client() -> GitHubClient {
    let mut builder = GitHubClientBuilder::new();
    builder.for_user_agent(USER_AGENT);
    let token = env::var("github_token");
    match token {
        Ok(t) =>
            builder
                .with_oauth_token(t.as_str())
                .build(),
        Err(_) => builder.build()
    }
}