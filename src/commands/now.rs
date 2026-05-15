use poise::CreateReply;
use serenity::all::{Colour, CreateEmbed, CreateEmbedFooter};

use crate::{commands::utils::Error, models::pagination::PaginatedQueue, queue::EventfulQueueKey};

use super::utils::Context;

// TODO: make a button to change pages instead of entering page number
#[poise::command(prefix_command, aliases("queue"))]
pub async fn now(ctx: Context<'_>, n: Option<usize>) -> Result<(), Error> {
    let n = n.unwrap_or(1);
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
    let k = EventfulQueueKey {
        guild_id,
        channel_id: channel_id.unwrap(),
    };
    let queue = lock.get_queue(&k).await;

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
    let paginated_queue = PaginatedQueue::new(queue, len, n);
    let pages = paginated_queue.total_pages();

    if n > pages {
        let embed = CreateEmbed::new()
            .title(format!(
                "❌ Number cannot be larger than total pages({})",
                pages
            ))
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
        .fields(paginated_queue.get_fields())
        .footer(CreateEmbedFooter::new(format!("Total Pages: {}", pages)))
        .color(Colour::from_rgb(0, 236, 255));
    ctx.send(CreateReply {
        embeds: vec![embed],
        ..Default::default()
    })
    .await?;

    Ok(())
}
