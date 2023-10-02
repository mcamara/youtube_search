use async_trait::async_trait;
use reqwest::{Client, Error, Response};

pub struct HttpClient {
    client: Client,
}

#[async_trait]
pub trait HttpClientTrait {
    fn new() -> Self;
    async fn get(&self, url: &str) -> Result<Response, Error>;
}

#[async_trait]
impl HttpClientTrait for HttpClient {
    fn new() -> Self {
        HttpClient {
            client: Client::new(),
        }
    }

    async fn get(&self, url: &str) -> Result<Response, Error> {
        self.client.get(url).send().await
    }
}
