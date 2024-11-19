// use futures::TryStreamExt;
// use reqwest::{header::{HeaderMap, HeaderValue}, StatusCode};
// use rspotify::{model::PlaylistId, prelude::{BaseClient, OAuthClient}, AuthCodeSpotify, ClientCredsSpotify};
// use songbird::{input::YoutubeDl, TrackEvent};
// use std::collections::VecDeque;
// use std::fmt::Display;

// use crate::{
//     commands::utils::Error, events::track_error_notifier::TrackErrorNotifier, models::spotify::Root,
// };

use super::utils::{Context, Error};

// struct Song {
//     song_name: String,
//     index: usize,
// }

// impl Display for Song {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{} - {}", self.index, self.song_name)
//     }
// }
// /// Plays music - pass the name of song.
#[poise::command(prefix_command)]
pub async fn playlist(_ctx: Context<'_>) -> Result<(), Error> {
    //     let (guild_id, channel_id) = {
    //         let guild = ctx.guild().expect("Guild only command");
    //         let channel_id = guild
    //             .voice_states
    //             .get(&ctx.author().id)
    //             .and_then(|voice_state| voice_state.channel_id);

    //         (guild.id, channel_id)
    //     };

    //     if let None = channel_id {
    //         ctx.say("Not in a voice chat.").await?;
    //         return Ok(());
    //     }

    //     let channel_id = channel_id.unwrap();
    //     let manager = songbird::get(&ctx.serenity_context())
    //         .await
    //         .expect("Songbird Voice client placed in at initialisation.")
    //         .clone();
    //     let http_client = ctx.data().hc.clone();

    //     let mut headers = HeaderMap::new();

    //     let id = url.split("/").last().unwrap();
    //     // let spotify = ClientCredsSpotify::with_config(creds, config);

    //     // spotify.request_token().await?;
    //         let spotify = AuthCodeSpotify::new(creds, oauth);

    //             let stream = spotify.playlist_items(PlaylistId::from_uri("uri"), Some("fields.name"), None);

    //              while let Some(item) = stream.try_next().await.unwrap() {
    //         println!("* {}", item.track.unwrap().name());
    //     }

    //     headers.insert(
    //         "Authorization",
    //         HeaderValue::from_str(&format!("Bearer {}", ctx.data().spotify_token.clone())).unwrap(),
    //     );
    //     println!("{:?}", headers);

    //     let response = http_client
    //         .get(&format!("https://api.spotify.com/v1/playlists/{}", id))
    //         .headers(headers)
    //         .send()
    //         .await?;
    //     println!("{}", response.status());
    //     if response.status() != StatusCode::OK {
    //         ctx.say(&format!("Error {}", response.status())).await?;
    //     }

    //     let body = response.json::<Root>().await.unwrap();

    //     let tl = body
    //         .tracks
    //         .items
    //         .into_iter()
    //         .enumerate()
    //         .map(|(index, e)| Song {
    //             index,
    //             song_name: format!(
    //                 "{} {}",
    //                 e.track.name,
    //                 e.track
    //                     .artists
    //                     .iter()
    //                     .map(|e| e.name.clone())
    //                     .collect::<Vec<_>>()
    //                     .join(" ")
    //             ),
    //         })
    //         .collect::<VecDeque<_>>();

    //     if let Ok(handler_lock) = manager.join(guild_id, channel_id).await {
    //         {
    //             let mut handler = handler_lock.lock().await;
    //             handler.add_global_event(TrackEvent::Error.into(), TrackErrorNotifier)
    //         };
    //         ctx.say(format!(
    //             "⏯️ **Now Playing** {}\n✍🏻 By {}",
    //             body.name, body.owner.display_name
    //         ))
    //         .await?;
    //         tokio::spawn(async move {
    //             let mut handler = handler_lock.lock().await;
    //             for t in tl {
    //                 let src = YoutubeDl::new_search(http_client.clone(), t.song_name);
    //                 handler.enqueue_input(src.clone().into()).await;
    //             }
    //         });
    //     }

    Ok(())
}
