use crate::error::Result;

pub struct HttpClient {
    client: reqwest::Client,
}

impl HttpClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn get(&self, _url: &str) -> Result<String> {
        todo!("Implement HTTP GET")
    }

    pub async fn post(&self, _url: &str, _body: &str) -> Result<String> {
        todo!("Implement HTTP POST")
    }
}