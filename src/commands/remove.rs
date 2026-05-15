use poise::CreateReply;
use serenity::all::{Colour, CreateEmbed};

use crate::queue::EventfulQueueKey;

use super::utils::{Context, Error};

#[poise::command(prefix_command)]
pub async fn remove(ctx: Context<'_>, n: u64) -> Result<(), Error> {
    if n <= 1 {
        let embed = CreateEmbed::new()
            .title("❌ Number must be greater than 1.")
            .color(Colour::from_rgb(255, 0, 0));
        ctx.send(CreateReply {
            embeds: vec![embed],
            ..Default::default()
        })
        .await?;
        return Ok(());
    }
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
    if handler_lock.queue().len() < n.try_into().unwrap() {
        let embed = CreateEmbed::new()
            .title("❌ Number cannot be larger than the queue size.")
            .color(Colour::from_rgb(255, 0, 0));
        ctx.send(CreateReply {
            embeds: vec![embed],
            ..Default::default()
        })
        .await?;
        return Ok(());
    }

    let track =
        { ctx.data().queue.read().await.get_queue(&k).await.unwrap()[(n - 1) as usize].clone() };

    handler_lock.queue().modify_queue(|queue| {
        queue.remove((n - 1) as usize);
    });
    {
        ctx.data()
            .queue
            .write()
            .await
            .remove(k, (n - 1) as usize)
            .await;
    }
    let embed = CreateEmbed::new()
        .title("✅ Removed Track")
        .field(format!("{} - {}", track.artist, track.name), "", false)
        .color(Colour::from_rgb(0, 255, 0));
    ctx.send(CreateReply {
        embeds: vec![embed],
        ..Default::default()
    })
    .await?;
    Ok(())
}
