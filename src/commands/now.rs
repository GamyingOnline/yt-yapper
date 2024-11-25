use poise::CreateReply;
use serenity::all::{Colour, CreateEmbed};

use crate::commands::utils::Error;

use super::utils::Context;

#[poise::command(prefix_command, aliases("queue"))]
pub async fn now(ctx: Context<'_>) -> Result<(), Error> {
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
    let lock = ctx.data().queue.read().await;
    let k = format!("{},{}", guild_id, channel_id.unwrap());
    let queue = lock.get(&k);

    if let None = queue {
        let embed = CreateEmbed::new()
            .title("❌ No music is playing.")
            .color(Colour::from_rgb(255, 0, 0));
        ctx.send(CreateReply {
            embeds: vec![embed],
            ..Default::default()
        })
        .await?;
        return Ok(());
    }
    let queue = queue.unwrap();

    let len = queue.len();
    if len == 0 {
        let embed = CreateEmbed::new()
            .title("❌ Queue is currently empty.")
            .color(Colour::from_rgb(255, 0, 0));
        ctx.send(CreateReply {
            embeds: vec![embed],
            ..Default::default()
        })
        .await?;
        return Ok(());
    }
    let embed = CreateEmbed::new()
        .title("📋 **Currently Playing**")
        .title("".to_string())
        .fields(queue.iter().enumerate().map(|(index, song)| {
            if index == 0 {
                (
                    format!(
                        "{}. {} - {}[{}] ⬅️",
                        index + 1,
                        song.artist,
                        song.name,
                        song.duration
                    ),
                    "",
                    false,
                )
            } else {
                (
                    format!(
                        "{}. {} - {}[{}]",
                        index + 1,
                        song.artist,
                        song.name,
                        song.duration
                    ),
                    "",
                    false,
                )
            }
        }))
        .color(Colour::from_rgb(0, 236, 255));
    ctx.send(CreateReply {
        embeds: vec![embed],
        ..Default::default()
    })
    .await?;

    Ok(())
}
