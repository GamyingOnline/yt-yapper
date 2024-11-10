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
        ctx.say("Not in a voice chat.").await?;
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
             ctx.say(format!("❌ Nothing to skip")).await?;
             return Ok(());
        }
        queue.skip()?;
    }
    ctx.say(format!("⏩ Skipped")).await?;
    Ok(())
}
