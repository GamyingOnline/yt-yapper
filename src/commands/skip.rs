use poise::CreateReply;
use serenity::all::{Colour, CreateEmbed};

use crate::commands::utils::Error;

use super::utils::Context;

#[poise::command(prefix_command, guild_only)]
pub async fn skip(ctx: Context<'_>, n: Option<usize>) -> Result<(), Error> {
    let (guild_id, channel_id) = {
        let guild = ctx.guild().expect("Guild only message");
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
    if let Ok(handler_lock) = manager.join(guild_id, channel_id).await {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();
        if queue.len() == 0 {
            let embed = CreateEmbed::new()
                .title("❌ Nothing to skip.")
                .color(Colour::from_rgb(255, 0, 0));
            ctx.send(CreateReply {
                embeds: vec![embed],
                ..Default::default()
            })
            .await?;
            return Ok(());
        }
        let n_times = if n.unwrap_or(1) >= queue.len() {
            queue.len()
        } else {
            n.unwrap_or(1)
        };
        let k = &format!("{},{}", guild_id.get(), channel_id.get());
        for _ in 0..n_times {
            queue.skip()?;
            let pop = {
                ctx.data()
                    .queue
                    .write()
                    .await
                    .get_mut(k)
                    .unwrap()
                    .pop_front()
            };
            if let None = pop {
                let embed = CreateEmbed::new()
                    .title(format!(
                        "⏩ Skipped {} {}",
                        n_times,
                        if n_times > 1 { "tracks" } else { "track" }
                    ))
                    .color(Colour::from_rgb(0, 255, 0));
                ctx.send(CreateReply {
                    embeds: vec![embed],
                    ..Default::default()
                })
                .await?;
                let next_track = {
                    ctx.data()
                        .queue
                        .read()
                        .await
                        .get(k)
                        .unwrap()
                        .front()
                        .cloned()
                };
                if let Some(next_track) = next_track {
                    let embed = CreateEmbed::new()
                        .title("**⏯️ Now Playing**")
                        .field(
                            next_track.artist,
                            format!("{} [{}]", next_track.name, next_track.duration),
                            true,
                        )
                        .image(next_track.thumbnail)
                        .color(Colour::from_rgb(0, 255, 0));
                    ctx.send(CreateReply {
                        embeds: vec![embed],
                        ..Default::default()
                    })
                    .await?;
                }
                return Ok(());
            }
        }
        let embed = CreateEmbed::new()
            .title(format!(
                "⏩ Skipped {} {}",
                n_times,
                if n_times > 1 { "tracks" } else { "track" }
            ))
            .color(Colour::from_rgb(0, 255, 0));
        ctx.send(CreateReply {
            embeds: vec![embed],
            ..Default::default()
        })
        .await?;

        let next_track = {
            ctx.data()
                .queue
                .read()
                .await
                .get(k)
                .unwrap()
                .front()
                .cloned()
        };

        if let Some(next_track) = next_track {
            let embed = CreateEmbed::new()
                .title("**⏯️ Now Playing**")
                .image(next_track.thumbnail)
                .field(
                    next_track.artist,
                    format!("{} [{}]", next_track.name, next_track.duration),
                    true,
                )
                .color(Colour::from_rgb(0, 255, 0));
            ctx.send(CreateReply {
                embeds: vec![embed],
                ..Default::default()
            })
            .await?;
        }
    }
    Ok(())
}
