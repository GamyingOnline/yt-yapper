use std::collections::VecDeque;

use songbird::{
    input::{Compose, YoutubeDl},
    TrackEvent,
};

use crate::{commands::utils::Error, events::track_error_notifier::TrackErrorNotifier};

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
        ctx.say("Not in a voice chat.").await?;
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
        handler.enqueue_input(src.clone().into()).await;
        match handler.queue().len() {
            1 => {
                ctx.say(format!(
                    "⏯️ **Now Playing**\n\n**Title** - _{}_\n**Artist** - {}\n**Channel** - {}\n",
                    track
                        .title
                        .clone()
                        .unwrap_or_default()
                        .chars()
                        .take(50)
                        .collect::<String>(),
                    track.artist.unwrap_or_default(),
                    track.channel.unwrap_or_default(),
                ))
                .await?;
                queues
                    .write()
                    .await
                    .get_mut(&k)
                    .unwrap()
                    .push_back((&track.title.clone().unwrap()).clone());
            }
            v => {
                ctx.say(format!(
                    "✅ **Queued** at #{}\n\n**Title** - _{}_\n**Artist** - {}\n**Channel** - {}",
                    v,
                    track
                        .title
                        .clone()
                        .unwrap_or_default()
                        .chars()
                        .take(50)
                        .collect::<String>(),
                    track.artist.unwrap_or_default(),
                    track.channel.unwrap_or_default(),
                ))
                .await?;
                queues
                    .write()
                    .await
                    .get_mut(&k)
                    .unwrap()
                    .push_back((&track.title.clone().unwrap()).clone());
            }
        }
        ctx.say(track.thumbnail.unwrap()).await?;
    }

    Ok(())
}
