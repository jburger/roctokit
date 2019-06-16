use crate::tests::{get_token_auth_client};

#[test]
fn it_retrieves_repositories_for_an_organization() {
    let client = get_token_auth_client();
    let organizations = client.repositories.for_org_name("octopusdeploy", None, None, None);
    assert_eq!(organizations.len(), 121);
}