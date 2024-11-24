# YT-Yapper

A feature-rich Discord music bot written in Rust that enables music playback and control in Discord voice channels.

## Features

- Music playback from various sources
- Playlist support
- Comprehensive playback controls:
  - Play/Pause
  - Skip tracks
  - Seek within tracks
  - Clear queue
  - Repeat mode
  - Now playing information
- Server-specific music queues
- Automatic voice channel state handling


## Installation

1. Ensure you have Rust installed on your system
2. Clone this repository:
```bash
git clone https://github.com/yourusername/yt-yapper.git
cd yt-yapper
```
3. Set up your Discord bot token as an environment variable:
```bash
export DISCORD_TOKEN=your_token_here
```
4. Build and run the bot:
```bash
cargo build --release
cargo run
```

## Dependencies

- [poise](https://crates.io/crates/poise) - Discord bot framework
- [songbird](https://crates.io/crates/songbird) - Voice and audio handling
- [tokio](https://crates.io/crates/tokio) - Async runtime
- [serenity](https://crates.io/crates/serenity) - Discord API wrapper
- [rspotify](https://crates.io/crates/rspotify) - Spotify integration
- [symphonia](https://crates.io/crates/symphonia) - Audio decoding

## License

This project is licensed under the terms included in the LICENSE file.
