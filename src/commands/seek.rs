use poise::CreateReply;
use serenity::all::{Colour, CreateEmbed};

use crate::{commands::utils::time_to_duration, queue::EventfulQueueKey};

use super::utils::{Context, Error};

#[poise::command(prefix_command)]
pub async fn seek(ctx: Context<'_>, time: String) -> Result<(), Error> {
    let duration = time_to_duration(&time);

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
    let handler = manager.get(guild_id).unwrap();
    let handler_lock = handler.lock().await;
    if let None = handler_lock.queue().current() {
        let embed = CreateEmbed::new()
            .title("❌ Nothing is playing.")
            .color(Colour::from_rgb(255, 0, 0));
        ctx.send(CreateReply {
            embeds: vec![embed],
            ..Default::default()
        })
        .await?;
        return Ok(());
    }
    let k = EventfulQueueKey {
        guild_id,
        channel_id,
    };
    let track = { ctx.data().queue.read().await.front(&k).await.cloned() };
    let track_duration = time_to_duration(&track.unwrap().duration);

    if track_duration < duration {
        let embed = CreateEmbed::new()
            .title("❌ Seek value cannot be greater than duration.")
            .color(Colour::from_rgb(255, 0, 0));
        ctx.send(CreateReply {
            embeds: vec![embed],
            ..Default::default()
        })
        .await?;
        return Ok(());
    }

    let _ = { handler_lock.queue().current().unwrap().seek(duration) };

    Ok(())
}
