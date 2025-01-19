use std::collections::VecDeque;

use poise::CreateReply;
use serenity::all::{Colour, CreateEmbed};
use songbird::{
    input::{Compose, YoutubeDl},
    TrackEvent,
};

use crate::{
    commands::utils::{duration_to_time, Error},
    events::track_error_notifier::TrackErrorNotifier,
    queue::{EventState, MusicQueueKey, QueueMessage},
    state::Track,
};

use super::utils::Context;

/// Plays music - pass the name of song.
#[poise::command(prefix_command, aliases("youtube"))]
pub async fn yt(ctx: Context<'_>, song_name: Vec<String>) -> Result<(), Error> {
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
    let key = MusicQueueKey {
        guild_id,
        channel_id,
    };
    {
        let (responder, response) = tokio::sync::oneshot::channel::<Option<VecDeque<Track>>>();
        ctx.data()
            .queue
            .send(QueueMessage::GetQueue { key, responder })
            .await
            .unwrap();
        let queue = response.await?;
        if let None = queue {
            ctx.data()
                .queue
                .send(QueueMessage::AddQueue { key })
                .await
                .unwrap();
        }
    }
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

        let track = Track {
            name: track_metadata.title.unwrap_or_default(),
            handle_uuid: track_handle.uuid().to_string(),
            artist: track_metadata.artist.unwrap_or_default(),
            duration: duration_to_time(track_metadata.duration.unwrap_or_default()),
            thumbnail: track_metadata.thumbnail.unwrap_or_default(),
            album: track_metadata.album.unwrap_or_default(),
            can_scrobble: false,
            from_playlist: false,
        };
        ctx.data()
            .queue
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
