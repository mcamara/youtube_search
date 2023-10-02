# YouTube Search

## Overview

This Rust library provides an asynchronous interface for interacting with YouTube channels and videos. It uses the [lemnoslife youtube API](https://yt.lemnoslife.com) to fetch data from YouTube without the use of an API key. Please consider supporting them if you find this library useful.

## Features

- Fetch a YouTube channel by name
- Get the latest videos from a channel
- Search for a video by its video ID

## Requirements

- Rust (latest stable version recommended)
- Tokio runtime
- `thiserror` crate for error handling

## Installation

Add the following line to your `Cargo.toml` file under the `[dependencies]` section:

```toml
youtube_search = "1.0.0"
```

## Usage
Here's a simple example demonstrating how to use the library:

```rust
use youtube_search::{
    find_youtube_channel,
    find_latest_videos,
    find_video,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = find_youtube_channel("ChannelName").await?;
    println!("Channel ID: {}", channel.channel_id);

    let videos = find_latest_videos(&channel, 5).await?;
    println!("Latest videos: {:?}", videos);

    let video = find_video("video_id_here".to_string()).await?;
    println!("Video Title: {}", video.title);

    Ok(())
}
```

## Documentation

### Modules
`src/lib.rs`
The main library file which provides functions for user interaction.

`src/youtube/channel.rs`
Defines the Channel struct and methods to initialize and fetch details.

`src/youtube/video.rs`
Defines the Video struct and methods to search for videos by ID.

`src/youtube/playlist.rs`
Defines the Playlist struct.

### Testing

This project uses the tokio test framework for asynchronous testing.

Run the test suite with:

```bash
cargo test
```

## Contributions

Feel free to open issues or pull requests if you have any improvements or fixes.

## License

MIT License. See `LICENSE` for details
