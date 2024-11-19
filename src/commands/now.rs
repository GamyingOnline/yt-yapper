
use crate::commands::utils::Error;

use super::utils::Context;

#[poise::command(prefix_command)]
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
        ctx.say("Not in a voice chat.").await?;
        return Ok(());
    }
    let lock = ctx.data().queue.read().await;
    let k = format!("{},{}", guild_id, channel_id.unwrap());
    let queue = lock.get(&k);

    if let None = queue {
        ctx.say("No music playing currently").await?;
        return Ok(());
    }
    let queue = queue.unwrap();

    let len = queue.len();
    if len == 0 {
        ctx.say("Queue is empty currently").await?;
        return Ok(());
    }
    ctx.say(&format!(
        "📋 **Currently Playing**\n\n{}",
        queue
            .iter()
            .enumerate()
            .map(|(index, song)| {
                if index == 0 {
                    format!("**{} - {} << **", index + 1, song)
                } else {
                    format!("{} - {}", index + 1, song)
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    ))
    .await?;
    Ok(())
}
