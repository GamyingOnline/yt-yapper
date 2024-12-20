use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
};

use ::serenity::async_trait;
use rustfm_scrobble::Scrobble;
use serenity::all::{ChannelId, Colour, Context, CreateEmbed, CreateMessage};
use songbird::{
    events::{Event, EventContext, EventHandler as VoiceEventHandler},
    tracks::PlayMode,
};
use sqlx::SqlitePool;
use tokio::sync::RwLock;

use crate::{
    scrobbler::{now_playing, scrobble},
    state::Track,
};
pub struct TrackErrorNotifier {
    pub queues: Arc<RwLock<HashMap<String, VecDeque<Track>>>>,
    pub channel_id: u64,
    pub guild_id: u64,
    pub message_channel_id: u64,
    pub context: Context,
    pub sql_conn: SqlitePool,
}

#[async_trait]
impl VoiceEventHandler for TrackErrorNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        let k = format!("{},{}", self.guild_id, self.channel_id);
        if let EventContext::Track(track_list) = ctx {
            let state = track_list.first();
            if let None = state {
                return None;
            }
            let (state, handle) = state.unwrap();

            let track = { self.queues.read().await.get(&k).unwrap().front().cloned() };
            if let Some(track) = track {
                if track.handle_uuid == handle.uuid().to_string() {
                    if state.playing == PlayMode::End || state.playing == PlayMode::Stop {
                        {
                            self.queues.write().await.get_mut(&k).unwrap().pop_front();
                        }

                        let users: Vec<u64> = ChannelId::new(self.channel_id)
                            .to_channel(&self.context)
                            .await
                            .unwrap()
                            .category()
                            .unwrap()
                            .members(&self.context)
                            .unwrap()
                            .iter()
                            .map(|member| member.user.id.get())
                            .collect();

                        if track.can_scrobble {
                            let old_song = Scrobble::new(&track.artist, &track.name, "");
                            scrobble(old_song, users.clone(), &self.sql_conn).await;
                        }

                        let new_track =
                            { self.queues.read().await.get(&k).unwrap().front().cloned() };

                        if let Some(new_track) = new_track {
                            let embed = CreateEmbed::new()
                                .title("**⏯️ Now Playing**")
                                .field(
                                    new_track.artist.clone(),
                                    format!("{} [{}]", new_track.name, new_track.duration),
                                    true,
                                )
                                .image(new_track.thumbnail)
                                .color(Colour::from_rgb(0, 255, 0));

                            ChannelId::new(self.message_channel_id)
                                .send_message(&self.context, CreateMessage::new().add_embed(embed))
                                .await
                                .expect("failed to send message");

                            if new_track.can_scrobble {
                                let song = Scrobble::new(
                                    &new_track.artist.clone(),
                                    &new_track.name.clone(),
                                    "",
                                );

                                now_playing(song, users, &self.sql_conn).await;
                            }

                            return None;
                        }
                    }
                }
            }
        }
        None
    }
}
