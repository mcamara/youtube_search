use reqwest::Response;
use serde::de::DeserializeOwned;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RequestError {
    #[error("Resource not found")]
    NotFound,
    #[error("{0}")]
    Other(String),
    #[error(transparent)]
    ResponseNotParsed(#[from] anyhow::Error),
    #[error(transparent)]
    Http(#[from] reqwest::Error),
}

pub async fn process_response<T: DeserializeOwned>(response: Response) -> Result<T, RequestError> {
    if response.status().is_success() {
        let parsed_data: T = response
            .json()
            .await
            .map_err(|e| RequestError::ResponseNotParsed(e.into()))?;
        return Ok(parsed_data);
    }

    Err(RequestError::Http(response.error_for_status().unwrap_err()))
}
