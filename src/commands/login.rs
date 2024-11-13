use std::time::Duration;

use poise::CreateReply;
use serenity::all::CreateEmbed;

use super::utils::Context;
use crate::commands::utils::Error;

#[poise::command(prefix_command)]
pub async fn login(ctx: Context<'_>) -> Result<(), Error> {
    let user_id = ctx.author().id.get();
    let scrobbler = ctx.data().scrobbler.clone();
    ctx.send(CreateReply {
        embeds: vec![CreateEmbed::new().url(scrobbler.start_login(user_id as i64))],
        ..Default::default()
    })
    .await?;
    ctx.data()
        .rx
        .recv_timeout(Duration::from_mins(2))
        .expect("Failed to create user");
    ctx.send(CreateReply {
        embeds: vec![CreateEmbed::new().description(format!(
            "✅ Successfully connected lastfm account for {}!",
            ctx.author().name
        ))],
        ..Default::default()
    })
    .await?;
    Ok(())
}
