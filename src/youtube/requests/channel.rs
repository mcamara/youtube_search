use super::utils::{process_response, RequestError};
use crate::utils::http_client::HttpClientTrait;
use serde::Deserialize;
use std::sync::Arc;
use url::Url;

#[derive(Deserialize)]
struct ChannelReturn {
    items: Vec<ChannelItemsReturn>,
}

#[derive(Deserialize)]
struct ChannelItemsReturn {
    id: Option<String>,
}

pub async fn retrieve_channel_id<T: HttpClientTrait>(
    name: &str,
    client: &Arc<T>,
) -> Result<String, RequestError> {
    let url = Url::parse_with_params("https://yt.lemnoslife.com/channels", &[("cId", name)])
        .map_err(|e| RequestError::Other(e.to_string()))?;

    let response = client.get(url.as_str()).await.map_err(RequestError::Http)?;
    let channel_data: ChannelReturn = process_response(response).await?;

    let channel_id = channel_data
        .items
        .first()
        .map(|item| item.id.clone())
        .ok_or(RequestError::NotFound);

    match channel_id {
        Ok(Some(channel_id)) => Ok(channel_id),
        Ok(None) => Err(RequestError::NotFound),
        Err(e) => Err(RequestError::ResponseNotParsed(e.into())),
    }
}
