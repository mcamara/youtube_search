use thiserror::Error;

use crate::utils::http_client::HttpClientTrait;
use std::sync::Arc;

use super::requests::video::retrieve_video_by_id;

/// A Video structure, it will contain all data regarding a video
#[derive(Debug)]
pub struct Video {
    pub id: String,
    pub title: String,
    pub description: String,
    pub published_at: String,
    pub thumbnail: String,
}

#[derive(Debug, Error)]
#[error("{msg}")]
pub struct VideoError {
    pub source: Option<anyhow::Error>,
    pub msg: String,
}

impl Video {
    pub fn new(
        id: String,
        title: String,
        description: String,
        published_at: String,
        thumbnail: String,
    ) -> Self {
        Self {
            id,
            title,
            description,
            published_at,
            thumbnail,
        }
    }

    /// Returns the url for this video
    pub fn url(&self) -> String {
        format!("https://www.youtube.com/watch?v={}", self.id)
    }

    pub async fn search_video_by_id<T: HttpClientTrait>(
        name: String,
        client: Arc<T>,
    ) -> Result<Self, VideoError> {
        retrieve_video_by_id(&name, client)
            .await
            .map_err(|e| VideoError {
                source: Some(e.into()),
                msg: "Failed to get video".to_owned(),
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::create_client_with_responses;

    #[tokio::test]
    async fn search_video_by_id_succeeds_with_valid_id() {
        let response = r#"{
            "items": [
                {
                    "snippet": {
                        "publishedAt": "2009-10-25T06:57:33Z",
                        "channelId": "UCuAXFkgsw1L7xaCfnd5JJOw",
                        "title": "Video Title",
                        "description": "Video Description",
                        "thumbnails": {
                            "high": {
                                "url": "https://i.ytimg.com/vi/dQw4w9WgXcQ/hqdefault.jpg"
                            }
                        }
                    }
                }
            ]
        }"#;

        let client = create_client_with_responses(vec![response]).await;
        let video = Video::search_video_by_id("dQw4w9WgXcQ".to_string(), client)
            .await
            .ok()
            .unwrap();

        assert_eq!(video.title, "Video Title");
        assert_eq!(video.description, "Video Description");
        assert_eq!(video.published_at, "2009-10-25T06:57:33Z");
        assert_eq!(
            video.thumbnail,
            "https://i.ytimg.com/vi/dQw4w9WgXcQ/hqdefault.jpg"
        );
        assert_eq!(video.url(), "https://www.youtube.com/watch?v=dQw4w9WgXcQ");
    }

    #[tokio::test]
    async fn search_video_by_id_fails_with_invalid_id() {
        let client = create_client_with_responses(vec!["{}"]).await;
        let video = Video::search_video_by_id("invalid_id".to_string(), client).await;

        assert!(video.is_err());
        assert_eq!(video.err().unwrap().msg, "Failed to get video");
    }
}
