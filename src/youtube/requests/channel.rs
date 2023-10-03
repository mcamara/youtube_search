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
    snippet: ChannelSnippetReturn,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ChannelSnippetReturn {
    channel_id: String,
    channel_title: String,
    channel_handle: String,
}

pub async fn retrieve_channel_id<T: HttpClientTrait>(
    handle: &str,
    client: &Arc<T>,
) -> Result<(String, String), RequestError> {
    let url = Url::parse_with_params(
        "https://yt.lemnoslife.com/search",
        &[
            ("q", handle),
            ("type", "channel"),
            ("part", "snippet"),
            ("maxResults", "10"),
        ],
    )
    .map_err(|e| RequestError::Other(e.to_string()))?;

    let response = client.get(url.as_str()).await.map_err(RequestError::Http)?;
    let channel_data: ChannelReturn = process_response(response).await?;

    match find_channel_by_handle(&channel_data.items, handle) {
        Ok(channel_snippet) => Ok((
            channel_snippet.channel_id.clone(),
            channel_snippet.channel_title.clone(),
        )),
        Err(e) => Err(e),
    }
}

fn find_channel_by_handle(
    channels: &[ChannelItemsReturn],
    target_handle: &str,
) -> Result<ChannelSnippetReturn, RequestError> {
    let target_handle = format!("@{}", target_handle);
    match channels
        .iter()
        .find(|&channel| channel.snippet.channel_handle == target_handle)
    {
        Some(channel) => Ok(channel.snippet.clone()),
        None => Err(RequestError::NotFound),
    }
}
