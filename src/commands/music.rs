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
    let src = YoutubeDl::new_search(http_client, song_name.join(" ").clone());
    let track = src.clone().aux_metadata().await?;
    if let Ok(handler_lock) = manager.join(guild_id, channel_id).await {
        let mut handler = handler_lock.lock().await;
        handler.add_global_event(TrackEvent::Error.into(), TrackErrorNotifier);
        handler.enqueue_input(src.clone().into()).await;
        match handler.queue().len() {
            1 => {
                ctx.say(format!(
                    "⏯️ **Now Playing**\n\n**Title** - _{}_\n**Artist** - {}\n**Channel** - {}\n",
                    track
                        .title
                        .unwrap_or_default()
                        .chars()
                        .take(50)
                        .collect::<String>(),
                    track.artist.unwrap_or_default(),
                    track.channel.unwrap_or_default(),
                ))
                .await?;
            }
            v => {
                ctx.say(format!(
                    "✅ **Queued** at #{}\n\n**Title** - _{}_\n**Artist** - {}\n**Channel** - {}",
                    v,
                    track
                        .title
                        .unwrap_or_default()
                        .chars()
                        .take(50)
                        .collect::<String>(),
                    track.artist.unwrap_or_default(),
                    track.channel.unwrap_or_default(),
                ))
                .await?;
            }
        }
        ctx.say(track.thumbnail.unwrap()).await?;
    }

    Ok(())
}
