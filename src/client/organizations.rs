use reqwest::{Client, Error};
use crate::model::{ Organization };

pub struct Organizations {
    pub client: Client,
    pub base_url: String
}

impl Organizations {
    pub fn find(&self, name: &str) -> Result<Organization, Error> {
        let url = self.base_url.as_str().replace("{org}", name);
        let result = self.client.get(url.as_str()).send();
        let mut response = result.unwrap();
        response.json::<Organization>()
    }
}