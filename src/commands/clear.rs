use poise::CreateReply;
use serenity::all::{Colour, CreateEmbed};

use crate::{
    commands::utils::Error,
    queue::{EventState, MusicQueueKey, QueueMessage},
};

use super::utils::Context;

/// Clears the whole queue
#[poise::command(prefix_command)]
pub async fn clear(ctx: Context<'_>) -> Result<(), Error> {
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
        let key = MusicQueueKey {
            guild_id,
            channel_id,
        };
        ctx.data()
            .queue
            .send(QueueMessage::Clear {
                key,
                event_state: EventState {
                    context: ctx.serenity_context().clone(),
                    channel_id,
                    text_channel_id: ctx.channel_id(),
                    sql_conn: ctx.data().sql_conn.clone(),
                },
            })
            .await
            .unwrap();
        queue.stop();
    }
    Ok(())
}
