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
        let n_times = if n.unwrap_or(1) >= queue.len() {
            queue.len()
        } else {
            n.unwrap_or(1)
        };
        let k = &format!("{},{}", guild_id.get(), channel_id.get());
        let mut skipped_songs = vec![];
        for i in 0..n_times {
            queue.skip()?;
            let pop = ctx.data().queue.write().await.get_mut(k).unwrap().pop_front();
            if let None = pop {
                ctx.say(format!(
                    "⏩ Skipped tracks: {}\n{}",
                    i,
                    skipped_songs
                        .iter()
                        .enumerate()
                        .map(|(i, song)| format!("{} - {}", i + 1, song))
                        .collect::<Vec<_>>()
                        .join("\n")
                ))
                .await?;
                return Ok(());
            }
            skipped_songs.push(pop.unwrap());
        }
        ctx.say(format!(
            "⏩ Skipped tracks: {}\n{}",
            n_times,
            skipped_songs
                .iter()
                .enumerate()
                .map(|(i, song)| format!("{} - {}", i + 1, song))
                .collect::<Vec<_>>()
                .join("\n")
        ))
        .await?;
    }
    Ok(())
}
