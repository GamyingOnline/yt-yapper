use poise::CreateReply;
use serenity::all::{Colour, CreateEmbed};
use songbird::tracks::LoopState;

use crate::queue::EventfulQueueKey;

use super::utils::{Context, Error};

#[poise::command(prefix_command, rename = "loop")]
pub async fn repeat(ctx: Context<'_>) -> Result<(), Error> {
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
    let track = { ctx.data().queue.read().await.front(k).await.cloned() };
    if handler_lock
        .queue()
        .current()
        .unwrap()
        .get_info()
        .await
        .unwrap()
        .loops
        == LoopState::Finite(0)
    {
        handler_lock
            .queue()
            .current()
            .unwrap()
            .enable_loop()
            .unwrap();
        let embed = CreateEmbed::new()
            .title("♾️ Looping this track")
            .description(track.unwrap().name)
            .color(Colour::from_rgb(0, 255, 0));
        ctx.send(CreateReply {
            embeds: vec![embed],
            ..Default::default()
        })
        .await?;
        return Ok(());
    }
    handler_lock
        .queue()
        .current()
        .unwrap()
        .disable_loop()
        .unwrap();
    let embed = CreateEmbed::new()
        .title("♾️ Disabled loop.")
        .description(track.unwrap().name)
        .color(Colour::from_rgb(255, 0, 0));
    ctx.send(CreateReply {
        embeds: vec![embed],
        ..Default::default()
    })
    .await?;
    Ok(())
}
