use thiserror::Error;

/// Main playlist for a channel, all its videos will be uploaded to this playlist
#[derive(Debug)]
pub struct Playlist {
    pub channel_id: String,
    pub playlist_id: String,
}

#[derive(Debug, Error)]
#[error("{msg}")]
pub struct PlaylistError {
    source: Option<anyhow::Error>,
    msg: String,
}

impl Playlist {
    pub fn new(channel_id: String, playlist_id: String) -> Self {
        Self {
            channel_id,
            playlist_id,
        }
    }
}
