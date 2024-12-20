use rustfm_scrobble::Scrobbler;

use crate::commands::utils::Error;
use crate::persistence::entities::user::User;

use super::utils::Context;

#[poise::command(prefix_command, aliases("login"), dm_only)]
pub async fn fmlogin(ctx: Context<'_>, username: String, password: String) -> Result<(), Error> {
    let api_key = std::env::var("LASTFM_API_KEY").expect("missing LASTFM_API_KEY");
    let api_secret = std::env::var("LASTFM_API_SECRET").expect("missing LASTFM_API_SECRET");
    let mut scrobbler = Scrobbler::new(&api_key, &api_secret);

    let token = scrobbler
        .authenticate_with_password(&username, &password)
        .expect("Incorrect creds.")
        .key
        .clone();
    let id = ctx.author().id.to_string().parse::<i64>()?;
    let user = User::new(id, token);
    sqlx::query!(
        "INSERT INTO user (id, token) VALUES (?, ?)",
        user.id,
        user.token
    )
    .execute(&ctx.data().sql_conn)
    .await?;

    ctx.say("discord पे, आपका lastfm टोकन प्राप्त हुआ!").await?;

    Ok(())
}
