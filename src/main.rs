#![feature(duration_constructors)]
use std::error::Error;

use commands::ping::ping;
use commands::skip::skip;
use commands::{login::login, music::music};

use persistence::connection::connect;
use poise::{serenity_prelude as serenity, PrefixFrameworkOptions};
use reqwest::Client as HttpClient;
use scrobbler::Scrobbler;
use songbird::SerenityInit;
use state::Data;

mod commands;
mod events;
mod persistence;
mod scrobbler;
mod server;
mod state;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt().init();
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![music(), ping(), skip(), login()],
            prefix_options: PrefixFrameworkOptions {
                prefix: Some(";".to_string()),
                ..Default::default()
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                let pool = connect().await?;
                Ok(Data {
                    hc: HttpClient::new(),
                    sqlite_conn: pool.clone(),
                    scrobbler: Scrobbler {
                        http_client: HttpClient::new(),
                        sqlite_conn: pool.clone(),
                        api_key: std::env::var("LASTFM_API_KEY").expect("missing LASTFM_API_KEY"),
                        token: std::env::var("LASTFM_TOKEN").expect("missing LASTFM_TOKEN"),
                    },
                })
            })
        })
        .build();

    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .register_songbird()
        .await?;

    client.start().await?;

    Ok(())
}
