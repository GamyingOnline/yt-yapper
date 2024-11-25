use std::error::Error;

use commands::music::music;
use commands::now::now;
use commands::pause::pause;
use commands::ping::ping;
use commands::play::playlist;
use commands::remove::remove;
use commands::seek::seek;
use commands::skip::skip;
use commands::{clear::clear, repeat::repeat};

use poise::{serenity_prelude as serenity, PrefixFrameworkOptions};
use reqwest::Client as HttpClient;
use songbird::SerenityInit;
use state::Data;

mod commands;
mod events;
mod models;
mod state;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt().init();
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                playlist(),
                music(),
                ping(),
                skip(),
                clear(),
                now(),
                repeat(),
                pause(),
                seek(),
                remove(),
            ],
            event_handler: |ctx, event, _, _| match event {
                serenity::FullEvent::VoiceStateUpdate { new, .. } => Box::pin(async move {
                    if new.user_id.to_string() == ctx.http.application_id().unwrap().to_string() {
                        let manager = songbird::get(&ctx)
                            .await
                            .expect("Songbird Voice client placed in at initialisation.")
                            .clone();
                        let handler = manager.get(new.guild_id.unwrap()).unwrap();
                        let handler_lock = handler.lock().await;
                        if let None = handler_lock.queue().current() {
                            return Ok(());
                        }
                        match new.mute {
                            true => {
                                handler_lock.queue().current().unwrap().pause().unwrap();
                            }
                            false => {
                                handler_lock.queue().current().unwrap().play().unwrap();
                            }
                        }
                    }
                    Ok(())
                }),
                _ => Box::pin(async move { Ok(()) }),
            },
            prefix_options: PrefixFrameworkOptions {
                prefix: Some(";".to_string()),
                ..Default::default()
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    hc: HttpClient::new(),
                    queue: Default::default(),
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
