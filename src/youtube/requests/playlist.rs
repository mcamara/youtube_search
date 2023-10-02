use super::utils::{process_response, RequestError};
use crate::utils::http_client::HttpClientTrait;
use serde::Deserialize;
use std::sync::Arc;
use url::Url;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct PlaylistReturn {
    items: Vec<PlaylistItemReturns>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct PlaylistItemReturns {
    content_details: PlaylistContentDetailsReturn,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct PlaylistContentDetailsReturn {
    related_playlists: RelatedPlaylistsReturn,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RelatedPlaylistsReturn {
    uploads: String,
}

pub async fn retrieve_main_playlist_id<T: HttpClientTrait>(
    channel_id: &str,
    client: &Arc<T>,
) -> Result<String, RequestError> {
    let url = Url::parse_with_params(
        "https://yt.lemnoslife.com/noKey/channels",
        &[("part", "contentDetails"), ("id", channel_id)],
    )
    .map_err(|e| RequestError::Other(e.to_string()))?;

    let response = client.get(url.as_str()).await.map_err(RequestError::Http)?;
    let playlist_data: PlaylistReturn = process_response(response).await?;

    let playlist_id = playlist_data
        .items
        .first()
        .map(|item| item.content_details.related_playlists.uploads.clone())
        .ok_or(RequestError::NotFound);

    match playlist_id {
        Ok(playlist_id) => Ok(playlist_id),
        Err(e) => Err(RequestError::ResponseNotParsed(e.into())),
    }
}
