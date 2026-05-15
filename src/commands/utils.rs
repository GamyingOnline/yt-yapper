use std::time::Duration;

use futures::future::join_all;
use serenity::all::{ChannelId, Colour, Context as SerenityContext, CreateEmbed, CreateMessage};

use crate::{
    persistence::SqlConn,
    scrobbler::Scrobbler,
    state::{Data, Track},
};

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

// Helper function to convert a `Duration` to a timestamp string
pub fn duration_to_time(duration: Duration) -> String {
    let mut secs = duration.as_secs() % (24 * 3600);
    let hours = secs / 3600;
    secs %= 3600;
    let mins = secs / 60;
    secs %= 60;

    return format!("{}:{:02}:{:02}", hours, mins, secs);
}

pub fn time_to_duration(time: &String) -> Duration {
    let split_str = time
        .split(':')
        .map(|x| x.parse::<u64>().ok().expect(&format!("cannot parse {}", x)))
        .collect::<Vec<_>>();

    let mut secs: u64 = 0;

    match split_str.len() {
        3 => {
            secs = split_str[2];
            secs += split_str[0] * 3600;
            secs += split_str[1] * 60;
        }
        2 => {
            secs = split_str[1];
            secs += split_str[0] * 60;
        }
        1 => {
            secs = split_str[0];
        }
        _ => {}
    }
    Duration::new(secs, 0)
}

pub async fn handle_playing(
    ctx: SerenityContext,
    text_channel_id: ChannelId,
    track: &Track,
    channel_id: ChannelId,
    sql_conn: &SqlConn,
) {
    let embed = CreateEmbed::new()
        .title("**⏯️ Now Playing**")
        .field(
            track.artist.to_string(),
            format!("{} [{}]", track.name, track.duration),
            true,
        )
        .image(track.thumbnail.to_string())
        .color(Colour::from_rgb(0, 255, 0));
    text_channel_id
        .send_message(&ctx, CreateMessage::new().add_embed(embed))
        .await
        .expect("Failed to send message");

    if track.can_scrobble {
        let channel = channel_id.to_channel(&ctx).await.unwrap();
        let guild = channel.guild().unwrap();
        let members = guild.members(&ctx).unwrap();

        let users_future: Vec<_> = members
            .iter()
            .map(|member| async { sql_conn.get_user(member.user.id.get() as i64).await })
            .collect();

        let users = join_all(users_future).await;
        for user in users {
            if user.is_some() {
                let api_key = std::env::var("LASTFM_API_KEY").expect("missing LASTFM_API_KEY");
                let api_secret =
                    std::env::var("LASTFM_API_SECRET").expect("missing LASTFM_API_SECRET");
                let mut scrobbler = Scrobbler::new(api_key, api_secret);
                let song = scrobbler
                    .track_to_scrobble(&track.artist, &track.name, &track.album)
                    .await;
                tokio::spawn(async move {
                    scrobbler.now_playing(&song, user.unwrap()).await;
                });
            }
        }
    }
}

pub async fn scrobble(
    ctx: SerenityContext,
    track: &Track,
    channel_id: ChannelId,
    sql_conn: &SqlConn,
) {
    let channel = channel_id.to_channel(&ctx).await.unwrap();
    let category = channel.guild().unwrap();
    let members = category.members(&ctx).unwrap();

    let users_future: Vec<_> = members
        .iter()
        .map(|member| async { sql_conn.get_user(member.user.id.get() as i64).await })
        .collect();

    let users = join_all(users_future).await;
    for user in users {
        if user.is_some() {
            let api_key = std::env::var("LASTFM_API_KEY").expect("missing LASTFM_API_KEY");
            let api_secret = std::env::var("LASTFM_API_SECRET").expect("missing LASTFM_API_SECRET");
            let mut scrobbler = Scrobbler::new(api_key, api_secret);
            let song = scrobbler
                .track_to_scrobble(&track.artist, &track.name, &track.album)
                .await;
            tokio::spawn(async move {
                scrobbler.scrobble(&song, user.unwrap()).await;
            });
        }
    }
}
