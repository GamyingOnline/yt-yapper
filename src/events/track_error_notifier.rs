use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
};

use ::serenity::async_trait;
use serenity::all::{ChannelId, Colour, Context, CreateEmbed, CreateMessage};
use songbird::{
    events::{Event, EventContext, EventHandler as VoiceEventHandler},
    tracks::PlayMode,
};
use tokio::sync::RwLock;

use crate::state::Track;
pub struct TrackErrorNotifier {
    pub queues: Arc<RwLock<HashMap<String, VecDeque<Track>>>>,
    pub channel_id: u64,
    pub guild_id: u64,
    pub message_channel_id: u64,
    pub context: Context,
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
                        let new_track =
                            { self.queues.read().await.get(&k).unwrap().front().cloned() };

                        if let Some(new_track) = new_track {
                            let embed = CreateEmbed::new()
                                .title("**⏯️ Now Playing**")
                                .field(
                                    new_track.artist,
                                    format!("{} [{}]", new_track.name, new_track.duration),
                                    true,
                                )
                                .image(new_track.thumbnail)
                                .color(Colour::from_rgb(0, 255, 0));
                            ChannelId::new(self.message_channel_id)
                                .send_message(&self.context, CreateMessage::new().add_embed(embed))
                                .await
                                .expect("failed to send message");
                            return None;
                        }
                    }
                }
            }
        }
        None
    }
}
