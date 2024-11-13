use poise::CreateReply;
use serenity::all::{Colour, CreateEmbed, CreateEmbedFooter};
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
            }
        }
    }

    Ok(())
}
