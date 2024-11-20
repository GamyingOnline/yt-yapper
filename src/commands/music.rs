use std::collections::VecDeque;

use poise::CreateReply;
use serenity::all::{Colour, CreateEmbed, CreateEmbedFooter};
use songbird::{
    input::{Compose, YoutubeDl},
    TrackEvent,
};

use crate::{
    commands::utils::Error, events::track_error_notifier::TrackErrorNotifier, state::Track,
};

use super::utils::Context;

/// Plays music - pass the name of song.
#[poise::command(prefix_command)]
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
            .description("❌ Not in a voice chat.")
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
            },
        );
        let track_handle = handler.enqueue_input(src.clone().into()).await;
        match handler.queue().len() {
            1 => {
                let embed = CreateEmbed::new()
                    .title("**⏯️ Now Playing**")
                    .field(
                        track.clone().artist.unwrap_or_default(),
                        track.clone().title.unwrap_or_default(),
                        true,
                    )
                    .description("".to_string())
                    .image(track.clone().thumbnail.unwrap())
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

                queues.write().await.get_mut(&k).unwrap().push_back(Track {
                    name: (&track.title.clone().unwrap()).clone(),
                    handle_uuid: track_handle.uuid().to_string(),
                });
            }
            v => {
                let embed = CreateEmbed::new()
                    .title(format!("**✅ Queued at position #{}**", v))
                    .field(
                        track.clone().artist.unwrap_or_default(),
                        track.clone().title.unwrap_or_default(),
                        true,
                    )
                    .description("".to_string())
                    .image(track.clone().thumbnail.unwrap())
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

                queues.write().await.get_mut(&k).unwrap().push_back(Track {
                    name: (&track.title.clone().unwrap()).clone(),
                    handle_uuid: track_handle.uuid().to_string(),
                });
            }
        }
    }

    Ok(())
}
