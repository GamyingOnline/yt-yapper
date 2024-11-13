use poise::CreateReply;
use serenity::all::CreateEmbed;

use crate::commands::utils::Error;

use super::utils::Context;

#[poise::command(prefix_command)]
pub async fn skip(ctx: Context<'_>) -> Result<(), Error> {
    let (guild_id, channel_id) = {
        let guild = ctx.guild().expect("Guild only message");
        let channel_id = guild
            .voice_states
            .get(&ctx.author().id)
            .and_then(|voice_state| voice_state.channel_id);

        (guild.id, channel_id)
    };

    if let None = channel_id {
        ctx.send(CreateReply {
            embeds: vec![CreateEmbed::new().description("❌ Not in a voice channel.")],
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
            ctx.send(CreateReply {
                embeds: vec![CreateEmbed::new().description("❌ Nothing to skip")],
                ..Default::default()
            })
            .await?;
            return Ok(());
        }
        queue.skip()?;
    }
    ctx.send(CreateReply {
        embeds: vec![CreateEmbed::new().description("⏩ Skipped")],
        ..Default::default()
    })
    .await?;
    Ok(())
}
