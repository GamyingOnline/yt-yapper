use std::collections::VecDeque;
use std::vec::Vec;

use poise::CreateReply;
use rustfm_scrobble::Scrobble;
use serenity::all::{Colour, CreateEmbed, CreateEmbedFooter};
use songbird::{
    input::{Compose, YoutubeDl},
    TrackEvent,
};

use crate::{
    commands::utils::{duration_to_time, Error},
    events::track_error_notifier::TrackErrorNotifier,
    scrobbler::now_playing,
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
    let http_client = ctx.data().hc.clone();
    let src = match song_name[0].starts_with("http") {
        true => YoutubeDl::new(http_client, song_name.join(" ").clone()),
        false => YoutubeDl::new_search(http_client, song_name.join(" ").clone()),
    };
    let queues = ctx.data().queue.clone();
    let k = format!("{},{}", guild_id, channel_id);
    {
        let mut lock = queues.write().await;
        let queue = lock.get(&k);
        if let None = queue {
            lock.insert(k.clone(), VecDeque::new());
        }
    }
    let track = src.clone().aux_metadata().await?;

    if let Ok(handler_lock) = manager.join(guild_id, channel_id).await {
        let mut handler = handler_lock.lock().await;
        handler.add_global_event(
            TrackEvent::End.into(),
            TrackErrorNotifier {
                channel_id: channel_id.get(),
                guild_id: guild_id.get(),
                queues: ctx.data().queue.clone(),
                context: ctx.serenity_context().clone(),
                message_channel_id: ctx.channel_id().get(),
                sql_conn: ctx.data().sql_conn.clone(),
            },
        );
        let track_handle = handler.enqueue_input(src.clone().into()).await;

        println!("{:?}", track);

        let mut track = Track {
            artist: track.clone().artist.unwrap_or_default(),
            name: track.clone().title.unwrap_or_default().clone(),
            handle_uuid: track_handle.uuid().to_string(),
            duration: duration_to_time(track.clone().duration.unwrap_or_default().clone()),
            thumbnail: track.clone().thumbnail.unwrap(),
            can_scrobble: false,
        };

        // let query = RecordingSearchQuery::query_builder()
        //     .artist(&track.clone().artist)
        //     .recording(&track.clone().name)
        //     .build();

        // let query_result = Recording::search(query).execute().await.unwrap();
        // println!("{:?}", query_result);
        // if query_result.count > 0 {
        //     let track_details = &query_result.entities[0];
        //     track.name = track_details.title.clone();
        //     let artists = track_details
        //         .artist_credit
        //         .clone()
        //         .unwrap()
        //         .iter()
        //         .map(|artist| artist.artist.name.clone())
        //         .collect::<Vec<String>>()
        //         .join(", ");
        //     track.artist = artists;
        //     // TODO: fetch coverart
        //     track.can_scrobble = true;
        // }

        match handler.queue().len() {
            1 => {
                let embed = CreateEmbed::new()
                    .title("**⏯️ Now Playing**")
                    .field(
                        track.clone().artist,
                        format!("{} [{}]", track.clone().name, track.duration),
                        true,
                    )
                    .image(track.clone().thumbnail)
                    .footer(
                        CreateEmbedFooter::new(format!("Requested by: {}", ctx.author().name))
                            .icon_url(ctx.author().avatar_url().unwrap_or_default()),
                    )
                    .color(Colour::from_rgb(0, 255, 0));
                ctx.send(CreateReply {
                    embeds: vec![embed],
                    ..Default::default()
                })
                .await?;

                queues
                    .write()
                    .await
                    .get_mut(&k)
                    .unwrap()
                    .push_back(track.clone());

                if track.can_scrobble {
                    let song = Scrobble::new(&track.artist, &track.name, "");

                    let users: Vec<u64> = ctx
                        .guild()
                        .unwrap()
                        .channels
                        .get(&channel_id)
                        .unwrap()
                        .members(&ctx)
                        .unwrap()
                        .iter()
                        .map(|member| member.user.id.get())
                        .collect();

                    now_playing(song, users, &ctx.data().sql_conn).await;
                }
            }
            v => {
                let embed = CreateEmbed::new()
                    .title(format!("**✅ Queued at position #{}**", v))
                    .field(
                        track.clone().artist,
                        format!("{} [{}]", track.clone().name, track.clone().duration),
                        true,
                    )
                    .thumbnail(track.clone().thumbnail)
                    .footer(
                        CreateEmbedFooter::new(format!("Requested by: {}", ctx.author().name))
                            .icon_url(ctx.author().avatar_url().unwrap_or_default()),
                    )
                    .color(Colour::from_rgb(0, 255, 0));
                ctx.send(CreateReply {
                    embeds: vec![embed],
                    ..Default::default()
                })
                .await?;

                queues.write().await.get_mut(&k).unwrap().push_back(track);
            }
        }
    }

    Ok(())
}
