use crate::commands::utils::Error;
use crate::persistence::entities::user::User;
use crate::scrobbler::Scrobbler;

use super::utils::Context;

#[poise::command(prefix_command, aliases("login"), dm_only)]
pub async fn fmlogin(ctx: Context<'_>, username: String, password: String) -> Result<(), Error> {
    let api_key = std::env::var("LASTFM_API_KEY").expect("missing LASTFM_API_KEY");
    let api_secret = std::env::var("LASTFM_API_SECRET").expect("missing LASTFM_API_SECRET");
    let token = Scrobbler::new(api_key, api_secret)
        .get_user_token(&username, &password)
        .await?;
    let user = User::new(ctx.author().id.get() as i64, token);
    user.save(&ctx.data().sql_conn.sql_conn).await?;

    ctx.reply("Saved LastFM Token").await?;
    Ok(())
}
