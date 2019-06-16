use crate::tests::{EXPECTED_ORG_NAME, get_token_auth_client};

#[test]
fn it_finds_organizations_token_auth() {
    let mut client = get_token_auth_client();
    let organization_result = client.organizations.get_by_name(EXPECTED_ORG_NAME);
    assert_eq!(organization_result.to_string(), "org: silentec 24204496  https://api.github.com/orgs/silentec");
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