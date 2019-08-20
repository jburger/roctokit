use crate::tests::{get_token_auth_client};

#[test]
fn it_retrieves_repositories_for_an_organization() {
    let client = get_token_auth_client();
    let repos = client.repositories.for_org_name("octopusdeploy", None, None, None);
    assert_eq!(repos.len(), 121);
}

#[test]
fn it_retrieves_repositories_for_a_user() {
    let client = get_token_auth_client();
    let repos = client.repositories.for_user("jburger");
    assert_eq!(repos.len(), 25);
}