use std::env;
use reqwest::{Client, Url};

pub struct Context {
    base_url : Url,        // base Url
    http_client: Client,   // 有連線池的http client
}

impl Context {
    pub fn load() -> Self {
        let base_url = env::var("API_BASE_URL")
            .unwrap_or("http://localhost:3030/".to_string());
        let base_url = Url::parse(&base_url).unwrap();
        let http_client = Client::new();
        Self {
            base_url,
            http_client,
        }
    }

    pub fn base_url(&self) -> &Url {          // getter的概念
        &self.base_url
    }

    pub fn http_client(&self) -> &Client {    // getter
        &self.http_client
    }
}
