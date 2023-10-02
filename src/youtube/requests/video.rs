use crate::youtube::requests::utils::RequestError;
use crate::{utils::http_client::HttpClientTrait, youtube::video::Video};
use serde::Deserialize;
use std::sync::Arc;
use url::Url;

use super::utils::process_response;

#[derive(Deserialize)]
struct VideoReturn {
    items: Vec<VideoItemReturns>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct VideoItemReturns {
    snippet: VideoSnippetReturn,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct VideoSnippetReturn {
    title: String,
    description: String,
    thumbnails: VideoThumbnailReturn,
    published_at: String,
    resource_id: Option<VideoResourceIdReturn>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct VideoResourceIdReturn {
    video_id: String,
}

#[derive(Deserialize)]
struct VideoThumbnailReturn {
    high: VideoThumbnailItemReturn,
}

#[derive(Deserialize)]
struct VideoThumbnailItemReturn {
    url: String,
}

pub async fn retrieve_latest_videos<T: HttpClientTrait>(
    playlist_id: &str,
    number_of_videos: i32,
    client: Arc<T>,
) -> Result<Vec<Video>, RequestError> {
    let url = Url::parse_with_params(
        "https://yt.lemnoslife.com/noKey/playlistItems",
        &[
            ("part", "snippet"),
            ("maxResults", number_of_videos.to_string().as_str()),
            ("playlistId", playlist_id),
        ],
    )
    .map_err(|e| RequestError::Other(e.to_string()))?;

    let response = client.get(url.as_str()).await.map_err(RequestError::Http)?;
    let video_data: VideoReturn = process_response::<VideoReturn>(response).await?;

    if video_data.items.is_empty() {
        return Err(RequestError::NotFound);
    }

    Ok(video_data
        .items
        .iter()
        .map(|item| {
            let resource = &item.snippet.resource_id;
            match resource {
                Some(resource) => Video::new(
                    resource.video_id.clone(),
                    item.snippet.title.clone(),
                    item.snippet.description.clone(),
                    item.snippet.published_at.clone(),
                    item.snippet.thumbnails.high.url.clone(),
                ),
                None => Video::new(
                    "".to_owned(),
                    item.snippet.title.clone(),
                    item.snippet.description.clone(),
                    item.snippet.published_at.clone(),
                    item.snippet.thumbnails.high.url.clone(),
                ),
            }
        })
        .collect())
}

pub async fn retrieve_video_by_id<T: HttpClientTrait>(
    video_id: &str,
    client: Arc<T>,
) -> Result<Video, RequestError> {
    let url = Url::parse_with_params(
        "https://yt.lemnoslife.com/noKey/videos",
        &[("part", "snippet"), ("id", video_id)],
    )
    .map_err(|e| RequestError::Other(e.to_string()))?;

    let response = client.get(url.as_str()).await.map_err(RequestError::Http)?;
    let video_return: VideoReturn = process_response::<VideoReturn>(response).await?;
    let video_data = video_return.items.first().ok_or(RequestError::NotFound)?;

    Ok(Video::new(
        video_id.to_string(),
        video_data.snippet.title.clone(),
        video_data.snippet.description.clone(),
        video_data.snippet.published_at.clone(),
        video_data.snippet.thumbnails.high.url.clone(),
    ))
}
