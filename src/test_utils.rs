use crate::utils::http_client::HttpClientTrait;
use async_trait::async_trait;
use reqwest::{Client, Error, Response};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct MockHttpClient {
    client: Client,
    responses: Arc<Mutex<Vec<String>>>,
}

#[cfg(test)]
impl MockHttpClient {
    pub fn new(responses: Vec<String>) -> Self {
        MockHttpClient {
            client: Client::new(),
            responses: Arc::new(Mutex::new(responses)),
        }
    }
}

#[async_trait]
impl HttpClientTrait for MockHttpClient {
    fn new() -> Self {
        MockHttpClient::new(vec!["".to_owned()])
    }

    async fn get(&self, _url: &str) -> Result<Response, Error> {
        let mut server = mockito::Server::new();

        let mut responses = self.responses.lock().await;
        let response = responses.pop().unwrap_or_default();

        let _mock = server
            .mock("GET", "/")
            .with_status(200)
            .with_body(response.clone())
            .create();

        self.client.get(&server.url()).send().await
    }
}

pub async fn create_client_with_responses(responses: Vec<&str>) -> Arc<MockHttpClient> {
    Arc::new(MockHttpClient::new(
        responses.into_iter().map(|s| s.to_string()).collect(),
    ))
}
