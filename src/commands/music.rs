use std::vec::Vec;

use poise::CreateReply;
use serenity::all::{Colour, CreateEmbed};
use songbird::{
    input::{Compose, YoutubeDl},
    TrackEvent,
};

use crate::{
    commands::utils::{duration_to_time, Error},
    events::{track_error_notifier::TrackErrorNotifier, track_queue_event::QueueEvent},
    queue::EventfulQueueKey,
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
    let mut src = match song_name[0].starts_with("http") {
        true => YoutubeDl::new(http_client, song_name.join(" ")),
        false => YoutubeDl::new_search(http_client, song_name.join(" ")),
    };
    let queues = &ctx.data().queue;
    let k = EventfulQueueKey {
        guild_id,
        channel_id,
    };
    {
        let mut lock = queues.write().await;
        let queue = lock.key_exists(&k).await;
        if !queue {
            lock.add_handler(
                QueueEvent {
                    channel_id,
                    guild_id,
                    text_channel_id: ctx.channel_id(),
                    context: ctx.serenity_context().clone(),
                    sql_conn: ctx.data().sql_conn.clone(),
                },
                &k,
            );
            lock.add_queue(k).await;
        }
    }
    let track = src.aux_metadata().await?;
    if let Ok(handler_lock) = manager.join(guild_id, channel_id).await {
        let mut handler = handler_lock.lock().await;
        handler.add_global_event(
            TrackEvent::End.into(),
            TrackErrorNotifier {
                channel_id,
                guild_id,
                queues: ctx.data().queue.clone(),
            },
        );
        let track_handle = handler.enqueue_input(src.into()).await;

        queues
            .write()
            .await
            .push(
                &k,
                Track {
                    name: track.title.unwrap(),
                    handle_uuid: track_handle.uuid().to_string(),
                    artist: track.artist.unwrap_or_default(),
                    duration: duration_to_time(track.duration.unwrap()),
                    thumbnail: track.thumbnail.unwrap(),
                },
            )
            .await;
    }

    Ok(())
}
