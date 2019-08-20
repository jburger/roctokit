use reqwest::{Client, Response};
use std::marker::PhantomData;
use regex::Regex;
use crate::clients::GitHubClientOptions;
use std::time::Duration;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT, AUTHORIZATION};

//todo: cache the client - learn enough about lifetimes to make a singleton
pub(crate) trait ApiClient {
    fn build_client(options: &GitHubClientOptions) -> Client {
        let builder =
            reqwest::ClientBuilder::new()
                .timeout(
                    Duration::from_secs(options.timeout_in_secs.unwrap_or(10)));

        let mut header_map = HeaderMap::new();
        header_map.append(
            USER_AGENT,
            HeaderValue::from_str(options.user_agent_string.as_str()).unwrap()
        );

        match &options.token {
            None => {}
            Some(token) => {
                header_map.append(
                    AUTHORIZATION,
                    HeaderValue::from_str(format!("token {}", token).as_str()).unwrap());
            }
        }

        builder
            .default_headers(header_map)
            .build()
            .unwrap()
    }

    fn get<T>(&self, options: &GitHubClientOptions, route: &str) -> T where for<'de> T: serde::Deserialize<'de> {
        let result = Self::build_client(options).get(route).send();
        match result {
            Ok(mut response) => {
                if !response.status().is_success() {
                    panic!(format!("Unable to access resource: {}", response.status()));
                }
                let deserialized = response.json::<T>();
                match deserialized {
                    Ok(resource) => resource,
                    Err(error) => {
                        panic!(format!("Unable to deserialize response body: {}", error));
                    }
                }
            },
            Err(error) => {
                panic!(format!("Unable to complete HTTP request: {}", error));
            }
        }
    }

    fn get_many<T>(&self, options: &GitHubClientOptions, route: &str, since: Option<usize>, limit: Option<usize>) -> Vec<T> where for<'de> T: serde::Deserialize<'de> {
        let mut paginator = Paginator::<T>::new(Self::build_client(options), route.to_string(), since, limit);
        let mut all_items = Vec::<T>::new();
        while let Some(mut new_items) = paginator.next() {
            all_items.append(&mut new_items);
        }
        all_items
    }
}

struct Paginator<T> where for<'de> T: serde::Deserialize<'de> {
    is_first_page: bool,
    count: usize,
    route: String,
    client: Client,
    since: usize,
    limit: usize,
    next_link: Option<String>,
    phantom: PhantomData<T>
}

struct PaginationLinks {
    next_link: Option<String>,
    prev_link: Option<String>,
    first_link: Option<String>,
    last_link: Option<String>
}

impl PaginationLinks {
    fn new(input: String) -> PaginationLinks {
        let components = input.split(",");
        let mut pagination_links = PaginationLinks {
            next_link: None,
            prev_link: None,
            first_link: None,
            last_link: None
        };

        for c in components {
            if c.contains("rel=\"next\"") {
                pagination_links.next_link = extract_link_from_hypermedia(c);
            }
            if c.contains("rel=\"prev\'") {
                pagination_links.prev_link = extract_link_from_hypermedia(c);
            }
            if c.contains("rel=\"first\"") {
                pagination_links.first_link = extract_link_from_hypermedia(c);
            }
            if c.contains("rel=\"last\"") {
                pagination_links.last_link = extract_link_from_hypermedia(c);
            }
        }

        return pagination_links;

        fn extract_link_from_hypermedia(c: &str) -> Option<String> {
            lazy_static! {
                static ref UNWANTED: Regex = Regex::new(r"[<>]").unwrap();
            }

            let url_components: Vec<&str> = c.split(";").collect();

            if url_components.len() < 2 {
                ()
            }

            let x = UNWANTED.replace_all(url_components[0], "").to_string();
            Some(x)
        }
    }
}

impl<T> Paginator<T> where for<'de> T: serde::Deserialize<'de> {
    fn new(client: Client, route: String, since: Option<usize>, limit: Option<usize>) -> Paginator<T> {
        Paginator {
            is_first_page: true,
            count: 0,
            route,
            client,
            since: since.unwrap_or(1),
            limit: limit.unwrap(),
            next_link: None,
            phantom: PhantomData
        }
    }

    fn get_next_link_from(&self, r: Response) -> Option<String> {
        if let Some(link_header) = r.headers().get("Link") {
            let pagination_link: Result<&str, _> = link_header.to_str();
            match pagination_link {
                Ok(header) => {
                    PaginationLinks::new(header.to_string()).next_link
                },
                Err(e) => panic!(e)
            }
        } else {
            None
        }
    }

    fn deserialize_new_items_from<R>(&self, response: &mut Response) -> Vec<R> where for<'de> R: serde::Deserialize<'de> {
        let deserialized = response.json();
        let new_items = match deserialized {
            Ok(result) => {
                result
            },
            Err(error) => {
                panic!(format!("Unable to deserialize response body: {}", error));
            }
        };
        new_items
    }
}

impl<T> Iterator for Paginator<T> where for<'de> T: serde::Deserialize<'de> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let response =
            if self.is_first_page {
                self.is_first_page = false;
                Some(self.client
                        .get(format!("{}?since={}", self.route, self.since).as_str())
                        .send()
                        .unwrap())
            } else {
                if let Some(next) = &self.next_link {
                    let n = next.to_string();
                    Some(self.client
                        .get(n.as_str())
                        .send()
                        .unwrap())
                } else {
                    None
                }
            };

        match response {
            Some(mut r) => {
                let mut new_items = self.deserialize_new_items_from(&mut r);
                let len = new_items.len();

                if len == 0 || self.count >= self.limit {
                    None
                } else if len + self.count > self.limit {
                    let remainder = self.limit - self.count;
                    self.count += remainder;
                    new_items.truncate(remainder);
                    Some(new_items)
                } else {
                    self.count += new_items.len();
                    self.next_link = self.get_next_link_from(r);
                    Some(new_items)
                }
            }
            None => None
        }
    }
}