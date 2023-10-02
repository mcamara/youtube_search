mod utils;
mod youtube;

#[cfg(test)]
pub mod test_utils;

use std::sync::Arc;
use utils::http_client::{HttpClient, HttpClientTrait};
use youtube::{
    channel::{Channel, ChannelError},
    video::{Video, VideoError},
};

/// Find a youtube channel by name
pub async fn find_youtube_channel(name: &str) -> Result<Channel, ChannelError> {
    let client = Arc::new(HttpClient::new());
    Channel::initialize(name.to_string(), client.clone()).await
}

/// Find latest videos from a channel, will return an error if the channel has no videos
pub async fn find_latest_videos(channel: &Channel, count: i32) -> Result<Vec<Video>, ChannelError> {
    let client = Arc::new(HttpClient::new());
    channel.get_latest_videos(count, client.clone()).await
}

/// Find a specific video on the platform by its id, will return an error if the video does not exist
pub async fn find_video(video_id: String) -> Result<Video, VideoError> {
    let client = Arc::new(HttpClient::new());
    Video::search_video_by_id(video_id, client.clone()).await
}
