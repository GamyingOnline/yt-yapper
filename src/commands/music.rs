use std::collections::VecDeque;

use poise::CreateReply;
use regex::Regex;
use serenity::all::{Colour, CreateEmbed};
use songbird::{
    input::{Compose, YoutubeDl},
    TrackEvent,
};
use spotify_rs::model::{track::Track as SpotifyTrack, PlayableItem};

use crate::{
    commands::utils::{duration_to_time, Error},
    events::track_error_notifier::TrackErrorNotifier,
    queue::{EventState, MusicQueueKey, QueueMessage},
    spotify::SpotifyClient,
    state::Track,
};

use super::utils::Context;

/// Plays music - pass the name of song.
#[poise::command(prefix_command, aliases("play"))]
pub async fn music(ctx: Context<'_>, song_name: Vec<String>) -> Result<(), Error> {
    let (guild_id, channel_id) = {
        let guild = ctx.guild().expect("Guild only command");
        let channel_id = guild
            .voice_states
            .get(&ctx.author().id)
            .and_then(|voice_state| voice_state.channel_id);

        (guild.id, channel_id)
    };

    if let None = channel_id {
        let embed = CreateEmbed::new()
            .title("❌ Not in a voice chat.")
            .color(Colour::from_rgb(255, 0, 0));
        ctx.send(CreateReply {
            embeds: vec![embed],
            ..Default::default()
        })
        .await?;
        return Ok(());
    }

    let channel_id = channel_id.unwrap();
    let manager = songbird::get(&ctx.serenity_context())
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let key = MusicQueueKey {
        guild_id,
        channel_id,
    };
    let queues = ctx.data().queue.clone();
    let http_client = ctx.data().hc.clone();

    {
        let (responder, response) = tokio::sync::oneshot::channel::<Option<VecDeque<Track>>>();
        queues
            .send(QueueMessage::GetQueue { key, responder })
            .await
            .unwrap();
        if response.await.unwrap().is_none() {
            queues.send(QueueMessage::AddQueue { key }).await.unwrap();
        }
    }

    let spotify_client_id = std::env::var("SPOTIFY_CLIENT_ID").expect("missing SPOTIFY_CLIENT_ID");
    let spotify_client_secret =
        std::env::var("SPOTIFY_CLIENT_SECRET").expect("missing SPOTIFY_CLIENT_SECRET");

    let mut spotify = SpotifyClient::new(spotify_client_id, spotify_client_secret);
    if song_name[0].starts_with("http") {
        let playlist_regex =
            Regex::new(r"https:\/\/open.spotify.com\/playlist\/[A-Za-z0-9]+(\?si=[a-f0-9]+)?")
                .unwrap();
        if playlist_regex.is_match(&song_name[0]) {
            let playlist = spotify
                .get_playlist(
                    song_name[0]
                        .strip_prefix("https://open.spotify.com/playlist/")
                        .unwrap()
                        .split("?")
                        .collect::<Vec<&str>>()[0]
                        .to_string(),
                )
                .await;
            if playlist.is_ok() {
                let unwrapped_playlist = playlist.unwrap();
                let songs = unwrapped_playlist
                    .tracks
                    .items
                    .iter()
                    .filter_map(|playlist_item| match &playlist_item.track {
                        PlayableItem::Track(track) => Some(track),
                        PlayableItem::Episode(_) => None,
                    })
                    .collect::<Vec<&SpotifyTrack>>();

                let embed = CreateEmbed::new()
                    .title(format!("✅ Queuing **{}** tracks", songs.len()))
                    .description(format!(
                        "from **{}**\nby **{}**",
                        unwrapped_playlist.name,
                        unwrapped_playlist.owner.display_name.unwrap_or_default()
                    ))
                    .thumbnail(unwrapped_playlist.images[0].url.to_string())
                    .color(Colour::from_rgb(0, 255, 0));
                ctx.send(CreateReply {
                    embeds: vec![embed],
                    ..Default::default()
                })
                .await?;

                for song in songs {
                    let mut track = Track {
                        can_scrobble: true,
                        album: song.album.name.to_string(),
                        artist: song.artists[0].name.to_string(),
                        name: song.name.to_string(),
                        thumbnail: song.album.images[0].url.to_string(),
                        from_playlist: true,
                        ..Default::default()
                    };

                    let mut src = YoutubeDl::new_search(
                        http_client.clone(),
                        format!("{} - {}", track.name, track.artist),
                    );

                    let track_metadata = src.aux_metadata().await?;
                    if let Ok(handler_lock) = manager.join(guild_id, channel_id).await {
                        let mut handler = handler_lock.lock().await;
                        handler.add_global_event(
                            TrackEvent::End.into(),
                            TrackErrorNotifier {
                                event_state: EventState {
                                    context: ctx.serenity_context().clone(),
                                    channel_id,
                                    text_channel_id: ctx.channel_id(),
                                    sql_conn: ctx.data().sql_conn.clone(),
                                },
                                guild_id,
                                queues: ctx.data().queue.clone(),
                            },
                        );
                        let track_handle = handler.enqueue_input(src.into()).await;

                        track.duration =
                            duration_to_time(track_metadata.duration.unwrap_or_default());
                        track.handle_uuid = track_handle.uuid().to_string();

                        queues
                            .send(QueueMessage::Push {
                                key,
                                value: track,
                                event_state: EventState {
                                    context: ctx.serenity_context().clone(),
                                    channel_id,
                                    text_channel_id: ctx.channel_id(),
                                    sql_conn: ctx.data().sql_conn.clone(),
                                },
                            })
                            .await
                            .unwrap();
                    }
                }
            }
        }
        let album_regex =
            Regex::new(r"https:\/\/open.spotify.com\/album\/[A-Za-z0-9]+(\?si=[a-f0-9]+)?")
                .unwrap();
        if album_regex.is_match(&song_name[0]) {
            let album = spotify
                .get_album(
                    song_name[0]
                        .strip_prefix("https://open.spotify.com/album/")
                        .unwrap()
                        .split("?")
                        .collect::<Vec<&str>>()[0]
                        .to_string(),
                )
                .await;
            if album.is_ok() {
                let unwrapped_album = album.unwrap();
                let songs = unwrapped_album.tracks.items;

                let embed = CreateEmbed::new()
                    .title(format!("✅ Queuing **{}** tracks", songs.len()))
                    .description(format!(
                        "from **{}**\nby **{}**",
                        unwrapped_album.name, unwrapped_album.artists[0].name
                    ))
                    .thumbnail(unwrapped_album.images[0].url.to_string())
                    .color(Colour::from_rgb(0, 255, 0));
                ctx.send(CreateReply {
                    embeds: vec![embed],
                    ..Default::default()
                })
                .await?;

                for song in songs {
                    let mut track = Track {
                        can_scrobble: true,
                        album: unwrapped_album.name.to_string(),
                        artist: song.artists[0].name.to_string(),
                        name: song.name.to_string(),
                        thumbnail: unwrapped_album.images[0].url.to_string(),
                        from_playlist: true,
                        ..Default::default()
                    };

                    let mut src = YoutubeDl::new_search(
                        http_client.clone(),
                        format!("{} - {}", track.name, track.artist),
                    );

                    let track_metadata = src.aux_metadata().await?;
                    if let Ok(handler_lock) = manager.join(guild_id, channel_id).await {
                        let mut handler = handler_lock.lock().await;
                        handler.add_global_event(
                            TrackEvent::End.into(),
                            TrackErrorNotifier {
                                event_state: EventState {
                                    context: ctx.serenity_context().clone(),
                                    channel_id,
                                    text_channel_id: ctx.channel_id(),
                                    sql_conn: ctx.data().sql_conn.clone(),
                                },
                                guild_id,
                                queues: ctx.data().queue.clone(),
                            },
                        );
                        let track_handle = handler.enqueue_input(src.into()).await;

                        track.duration =
                            duration_to_time(track_metadata.duration.unwrap_or_default());
                        track.handle_uuid = track_handle.uuid().to_string();

                        queues
                            .send(QueueMessage::Push {
                                key,
                                value: track,
                                event_state: EventState {
                                    context: ctx.serenity_context().clone(),
                                    channel_id,
                                    text_channel_id: ctx.channel_id(),
                                    sql_conn: ctx.data().sql_conn.clone(),
                                },
                            })
                            .await
                            .unwrap();
                    }
                }
            }
        } else {
            let embed = CreateEmbed::new()
                .title("**❌ Only Spotify Playlist/Album links are supported.")
                .description("For YouTube links use `;yt`")
                .color(Colour::from_rgb(255, 0, 0));
            ctx.send(CreateReply {
                embeds: vec![embed],
                ..Default::default()
            })
            .await?;
        }
        return Ok(());
    }

    let songs = spotify.get_track(song_name.join(" ")).await;
    let mut track = Track::default();
    if songs.is_ok() {
        let song = &songs.unwrap().tracks.unwrap().items[0];
        track.can_scrobble = true;
        track.album = song.album.name.to_string();
        track.artist = song.artists[0].name.to_string();
        track.name = song.name.to_string();
        track.thumbnail = song.album.images[0].url.to_string();
        track.from_playlist = false;
    }

    let mut src = YoutubeDl::new_search(http_client, format!("{} - {}", track.name, track.artist));

    let track_metadata = src.aux_metadata().await?;
    if let Ok(handler_lock) = manager.join(guild_id, channel_id).await {
        let mut handler = handler_lock.lock().await;
        handler.add_global_event(
            TrackEvent::End.into(),
            TrackErrorNotifier {
                event_state: EventState {
                    context: ctx.serenity_context().clone(),
                    channel_id,
                    text_channel_id: ctx.channel_id(),
                    sql_conn: ctx.data().sql_conn.clone(),
                },
                guild_id,
                queues: ctx.data().queue.clone(),
            },
        );
        let track_handle = handler.enqueue_input(src.into()).await;

        track.duration = duration_to_time(track_metadata.duration.unwrap_or_default());
        track.handle_uuid = track_handle.uuid().to_string();

        queues
            .send(QueueMessage::Push {
                key,
                value: track,
                event_state: EventState {
                    context: ctx.serenity_context().clone(),
                    channel_id,
                    text_channel_id: ctx.channel_id(),
                    sql_conn: ctx.data().sql_conn.clone(),
                },
            })
            .await
            .unwrap();
    }

    Ok(())
}
