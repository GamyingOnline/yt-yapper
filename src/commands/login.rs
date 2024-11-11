use super::utils::Context;
use crate::{commands::utils::Error, scrobbler::Scrobbler};

#[poise::command(prefix_command)]
pub async fn login(ctx: Context<'_>) -> Result<(), Error> {
    let user_id = ctx.author().id.get();
    let scrobbler = ctx.data().scrobbler.clone();
    ctx.say(scrobbler.start_login()).await?;
    Ok(())
}
