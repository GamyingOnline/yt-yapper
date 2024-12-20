use poise::CreateReply;
use rustfm_scrobble::Scrobble;
use serenity::all::{Colour, CreateEmbed};

use crate::{
    commands::utils::Error,
    scrobbler::{now_playing, scrobble},
};

use super::utils::Context;

#[poise::command(prefix_command, guild_only, track_edits)]
pub async fn skip(ctx: Context<'_>, n: Option<usize>) -> Result<(), Error> {
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

    if let Ok(handler_lock) = manager.join(guild_id, channel_id).await {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();
        if queue.len() == 0 {
            let embed = CreateEmbed::new()
                .title("❌ Nothing to skip.")
                .color(Colour::from_rgb(255, 0, 0));
            ctx.send(CreateReply {
                embeds: vec![embed],
                ..Default::default()
            })
            .await?;
            return Ok(());
        }
        let n_times = if n.unwrap_or(1) >= queue.len() {
            queue.len()
        } else {
            n.unwrap_or(1)
        };
        let k = &format!("{},{}", guild_id.get(), channel_id.get());
        let old_track = {
            ctx.data()
                .queue
                .read()
                .await
                .get(k)
                .unwrap()
                .front()
                .unwrap()
                .clone()
        };
        for _ in 0..n_times {
            queue.skip()?;
            let pop = {
                ctx.data()
                    .queue
                    .write()
                    .await
                    .get_mut(k)
                    .unwrap()
                    .pop_front()
            };
            if let None = pop {
                let embed = CreateEmbed::new()
                    .title(format!(
                        "⏩ Skipped {} {}",
                        n_times,
                        if n_times > 1 { "tracks" } else { "track" }
                    ))
                    .color(Colour::from_rgb(0, 255, 0));
                ctx.send(CreateReply {
                    embeds: vec![embed],
                    ..Default::default()
                })
                .await?;
                let next_track = {
                    ctx.data()
                        .queue
                        .read()
                        .await
                        .get(k)
                        .unwrap()
                        .front()
                        .cloned()
                };
                if let Some(next_track) = next_track {
                    let embed = CreateEmbed::new()
                        .title("**⏯️ Now Playing**")
                        .field(
                            next_track.artist.clone(),
                            format!("{} [{}]", next_track.name, next_track.duration),
                            true,
                        )
                        .image(next_track.thumbnail)
                        .color(Colour::from_rgb(0, 255, 0));
                    ctx.send(CreateReply {
                        embeds: vec![embed],
                        ..Default::default()
                    })
                    .await?;

                    let song = Scrobble::new(&next_track.artist, &next_track.name, "");

                    let channel_id = ctx
                        .guild()
                        .unwrap()
                        .voice_states
                        .get(&ctx.author().id)
                        .and_then(|voice_state| voice_state.channel_id)
                        .unwrap();

                    if next_track.can_scrobble {
                        let users: Vec<u64> = ctx
                            .guild()
                            .unwrap()
                            .channels
                            .get(&channel_id)
                            .unwrap()
                            .members(&ctx)
                            .unwrap()
                            .iter()
                            .map(|member| member.user.id.get())
                            .collect();

                        now_playing(song, users, &ctx.data().sql_conn).await;
                    }
                }
                return Ok(());
            }
        }
        let embed = CreateEmbed::new()
            .title(format!(
                "⏩ Skipped {} {}",
                n_times,
                if n_times > 1 { "tracks" } else { "track" }
            ))
            .color(Colour::from_rgb(0, 255, 0));
        ctx.send(CreateReply {
            embeds: vec![embed],
            ..Default::default()
        })
        .await?;

        let old_song = Scrobble::new(&old_track.artist, &old_track.name, "");

        let channel_id = ctx
            .guild()
            .unwrap()
            .voice_states
            .get(&ctx.author().id)
            .and_then(|voice_state| voice_state.channel_id)
            .unwrap();

        let users: Vec<u64> = ctx
            .guild()
            .unwrap()
            .channels
            .get(&channel_id)
            .unwrap()
            .members(&ctx)
            .unwrap()
            .iter()
            .map(|member| member.user.id.get())
            .collect();

        if old_track.can_scrobble {
            scrobble(old_song, users.clone(), &ctx.data().sql_conn).await;
        }

        let next_track = {
            ctx.data()
                .queue
                .read()
                .await
                .get(k)
                .unwrap()
                .front()
                .cloned()
        };

        if let Some(next_track) = next_track {
            let embed = CreateEmbed::new()
                .title("**⏯️ Now Playing**")
                .image(next_track.thumbnail)
                .field(
                    next_track.artist.clone(),
                    format!("{} [{}]", next_track.name, next_track.duration),
                    true,
                )
                .color(Colour::from_rgb(0, 255, 0));
            ctx.send(CreateReply {
                embeds: vec![embed],
                ..Default::default()
            })
            .await?;

            if next_track.can_scrobble {
                let song = Scrobble::new(&next_track.artist, &next_track.name, "");

                now_playing(song, users, &ctx.data().sql_conn).await;
            }
        }
    }
    Ok(())
}
