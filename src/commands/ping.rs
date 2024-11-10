use crate::commands::utils::Error;

use super::utils::Context;

#[poise::command(prefix_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Pong!".to_string()).await?;
    Ok(())
}
