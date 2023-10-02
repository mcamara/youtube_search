use crate::utils::http_client::HttpClientTrait;
use crate::youtube::requests::channel::retrieve_channel_id;
use std::sync::Arc;
use thiserror::Error;

use super::playlist::Playlist;
use super::requests::playlist::retrieve_main_playlist_id;
use super::requests::video::retrieve_latest_videos;
use super::video::Video;

// A youtube channel with some useful data
#[derive(Debug)]
pub struct Channel {
    pub name: String,
    pub channel_id: String,
}

#[derive(Debug, Error)]
#[error("{msg}")]
pub struct ChannelError {
    pub source: Option<anyhow::Error>,
    pub msg: String,
}

impl Channel {
    fn new(name: String, channel_id: String) -> Self {
        Self { name, channel_id }
    }

    pub async fn initialize<T: HttpClientTrait>(
        name: String,
        client: Arc<T>,
    ) -> Result<Self, ChannelError> {
        let channel_id = retrieve_channel_id(&name, &client)
            .await
            .map_err(|e| ChannelError {
                source: Some(e.into()),
                msg: "Failed to get channel id".to_owned(),
            })?;

        Ok(Self::new(name, channel_id))
    }

    async fn get_main_playlist_id<T: HttpClientTrait>(
        &self,
        client: Arc<T>,
    ) -> Result<Playlist, ChannelError> {
        let playlist_id = retrieve_main_playlist_id(&self.channel_id, &client)
            .await
            .map_err(|e| ChannelError {
                source: Some(e.into()),
                msg: "Failed to get playlist id".to_owned(),
            })?;

        Ok(Playlist::new(self.channel_id.clone(), playlist_id))
    }

    pub async fn get_latest_videos<T: HttpClientTrait>(
        &self,
        number_of_videos: i32,
        client: Arc<T>,
    ) -> Result<Vec<Video>, ChannelError> {
        let playlist = self.get_main_playlist_id(client.clone()).await?;
        retrieve_latest_videos(&playlist.playlist_id, number_of_videos, client)
            .await
            .map_err(|e| ChannelError {
                source: Some(e.into()),
                msg: "Failed to get videos from channel".to_owned(),
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::create_client_with_responses;

    #[tokio::test]
    async fn channel_initialization_succeeds_with_valid_id() {
        let client = create_client_with_responses(vec![
            r#"{"items": [
            {
                "id": "id_channel1"
            }
        ]}"#,
        ])
        .await;
        let channel = Channel::initialize("channel1".to_string(), client)
            .await
            .ok()
            .unwrap();
        assert_eq!(channel.channel_id, "id_channel1");
        assert_eq!(channel.name, "channel1");
    }

    #[tokio::test]
    async fn channel_initialization_fails_with_malformed_response() {
        let client = create_client_with_responses(vec![
            r#"{"another": [
            {
                "id": "id_channel1"
            }
        ]}"#,
        ])
        .await;
        let channel = Channel::initialize("channel1".to_string(), client).await;
        assert!(channel.is_err());
    }

    #[tokio::test]
    async fn channel_initialization_fails_with_null_id() {
        let client = create_client_with_responses(vec![
            r#"{"items": [
            {
                "id": null
            }
        ]}"#,
        ])
        .await;
        let channel = Channel::initialize("channel1".to_string(), client).await;
        assert!(channel.is_err());
    }

    #[tokio::test]
    async fn main_playlist_is_found_for_a_channel() {
        let channel = Channel::new("channel1".to_string(), "id_channel1".to_string());
        let client = create_client_with_responses(vec![
            r#"{"items": [
            {
                "contentDetails": {
                    "relatedPlaylists": {
                        "uploads": "playlist_id1"
                    }
                }
            }
        ]}"#,
        ])
        .await;
        let playlist = channel.get_main_playlist_id(client).await.ok().unwrap();
        assert_eq!(playlist.channel_id, "id_channel1");
        assert_eq!(playlist.playlist_id, "playlist_id1");
    }

    #[tokio::test]
    async fn main_playlist_search_fails_if_channel_do_not_have_any_playlist() {
        let channel = Channel::new("channel1".to_string(), "id_channel1".to_string());
        let client = create_client_with_responses(vec![
            r#"{"items": [
            {
                "contentDetails": {
                    "relatedPlaylists": {
                        "uploads": null
                    }
                }
            }
        ]}"#,
        ])
        .await;
        let result = channel.get_main_playlist_id(client).await;
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().msg, "Failed to get playlist id");
    }

    #[tokio::test]
    async fn videos_from_channel_are_returned() {
        let channel = Channel::new("channel1".to_string(), "id_channel1".to_string());
        let playlist_response = r#"{"items": [
            {
                "contentDetails": {
                    "relatedPlaylists": {
                        "uploads": "playlist_id1"
                    }
                }
            }
        ]}"#;

        let video_response = r#"{
            "items": [
                {
                    "snippet": {
                        "publishedAt": "2023-09-21T17:02:18Z",
                        "title": "Video Title 1",
                        "description": "Description video 1",
                        "thumbnails": {
                            "high": {
                                "url": "https://i.ytimg.com/vi/dQw4w9WgXcQ/hqdefault.jpg"
                            }
                        },
                        "resourceId": {
                            "videoId": "dQw4w9WgXcQ"
                        }
                    }
                },
                {
                    "snippet": {
                        "publishedAt": "2023-09-18T18:20:58Z",
                        "title": "Video Title 2",
                        "description": "Description video 2",
                        "thumbnails": {
                            "high": {
                                "url": "https://i.ytimg.com/vi/dQw4w9WgXcQ/hqdefault.jpg"
                            }
                        },
                        "resourceId": {
                            "videoId": "dQw4w9WgXcQ"
                        }
                    }
                }
            ]
        }"#;

        let client = create_client_with_responses(vec![video_response, playlist_response]).await;

        let videos = channel.get_latest_videos(2, client).await.unwrap();

        assert_eq!(videos.len(), 2);
        let video1 = videos.get(0).unwrap();
        assert_eq!(video1.url(), "https://www.youtube.com/watch?v=dQw4w9WgXcQ");
        assert_eq!(video1.title, "Video Title 1");
        assert_eq!(video1.description, "Description video 1");
    }

    #[tokio::test]
    async fn videos_from_channel_cannot_be_retrieved_when_response_is_empty() {
        let channel = Channel::new("channel1".to_string(), "id_channel1".to_string());
        let playlist_response = r#"{"items": [
            {
                "contentDetails": {
                    "relatedPlaylists": {
                        "uploads": "playlist_id1"
                    }
                }
            }
        ]}"#;

        let video_response = r#"{"items": []}"#;
        let client = create_client_with_responses(vec![video_response, playlist_response]).await;

        let videos = channel.get_latest_videos(1, client).await;

        assert!(videos.is_err());
        assert_eq!(
            videos.err().unwrap().msg,
            "Failed to get videos from channel"
        );
    }
}
