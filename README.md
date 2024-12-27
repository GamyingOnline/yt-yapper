# YT-Yapper

YT-Yapper is a feature-rich Discord music bot written in Rust. It provides seamless music playback and advanced controls while integrating Last.fm scrobbling and Spotify track data for a complete music experience.

## Features

- Music playback with search and URL support
- Comprehensive playback controls:
  - Play/Pause
  - Skip tracks
  - Seek within tracks
  - Clear queue
  - Remove specific tracks
  - Single track repeat mode with toggle
  - Now playing information with rich embeds and thumbnails
  - Automatic track progression
  - Spotify playlist support
- Server-specific features:
  - Per-server music queues
  - Server mute synchronization
  - Automatic voice channel state handling
- Last.fm Integration:
  - Automatic song scrobbling on track completion
  - Real-time now playing updates
  - Voice channel member tracking
  - Secure DM-based authentication
  - Multi-user support with session persistence
- SQLite persistence for Last.fm user tokens
- Spotify Integration: Fetches detailed track metadata using the spotify-rs crate.

## Installation

### 1. Prerequisites:

- Ensure Rust is installed on your system.
- Obtain a Discord bot token by creating a new application in the Discord Developer Portal.
- Set up your Last.fm API credentials for scrobbling.
- Set up Spotify API credentials for enhanced track metadata.

### 2. Clone the Repository:

```bash
git clone https://github.com/gamyingonline/yt-yapper.git
cd yt-yapper
```

### 3. Configuration:

- Rename .env.example to .env and fill in the required details:

```
    DATABASE_URL=database_url
    DISCORD_TOKEN=your_discord_token
    LASTFM_API_KEY=your_lastfm_api_key
    LASTFM_SECRET=your_lastfm_secret
    SPOTIFY_CLIENT_ID=your_spotify_client_id
    SPOTIFY_CLIENT_SECRET=your_spotify_client_secret
```

### 4. Build and Run:

```bash
cargo install sqlx
cargo sqlx prepare
cargo build --release
cargo run --release
```

## Dependencies

- [poise](https://crates.io/crates/poise) v0.6.1 - Discord bot framework
- [songbird](https://crates.io/crates/songbird) v0.4.3 - Voice and audio handling
- [tokio](https://crates.io/crates/tokio) v1.41.1 - Async runtime
- [serenity](https://crates.io/crates/serenity) v0.12.2 - Discord API wrapper
- [spotify-rs](https://crates.io/crates/spotify-rs) v0.3.14 - Spotify integration
- [symphonia](https://crates.io/crates/symphonia) v0.5.2 - Audio decoding
- [rustfm-scrobble](https://crates.io/crates/rustfm-scrobble) v1.1.1 - Last.fm integration
- [sqlx](https://crates.io/crates/sqlx) v0.8.2 - SQLite database integration
- [dotenv](https://crates.io/crates/dotenv) v0.15.0 - Environment configuration

## Usage

Once the bot is running, invite it to your Discord server using the OAuth2 URL generated in the Discord Developer Portal. Use the following commands with the prefix `;`

- `;play <playlist url or search>`: Play a track from a search query (searches from spotify) or a spotify playlist. (alias: `;music`)
- `;yt <url or search>`: Play a track from a url or search query (searches from yt). (alias: `;youtube`)
- `;pause`: Pause the current track.
- `;resume`: Resume playback.
- `;skip [<number of tracks>]`: Skip to the next track in the queue.
- `;queue`: Display the current queue.
- `;clear`: Clear the queue.
- `;now`: Show information about the currently playing track.
- `;seek <time>`: Seek within current track (format: HH:MM:SS, MM:SS, or SS)
- `;repeat`: Toggle repeat mode. (alias: `;loop`)
- `;fmlogin <username> <password>`: Set up Last.fm integration (DM only, alias: `;login`)

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request with your changes. Ensure your code adheres to the existing style.

## License

This project is licensed under the BSD-3-Clause License. See the [LICENSE](https://github.com/GamyingOnline/yt-yapper/blob/main/LICENSE) file for details.
