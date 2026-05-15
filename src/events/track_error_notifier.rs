use std::sync::Arc;

use ::serenity::async_trait;
use serenity::all::{ChannelId, GuildId};
use songbird::{
    events::{Event, EventContext, EventHandler as VoiceEventHandler},
    tracks::PlayMode,
};
use tokio::sync::RwLock;

use crate::{
    queue::{EventfulQueue, EventfulQueueKey},
    state::Track,
};
pub struct TrackErrorNotifier {
    pub queues: Arc<RwLock<EventfulQueue<Track>>>,
    pub channel_id: ChannelId,
    pub guild_id: GuildId,
}

#[async_trait]
impl VoiceEventHandler for TrackErrorNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        let k = EventfulQueueKey {
            guild_id: self.guild_id,
            channel_id: self.channel_id,
        };
        if let EventContext::Track(track_list) = ctx {
            let state = track_list.first();
            if let None = state {
                return None;
            }
            let (state, handle) = state.unwrap();

            let track = { self.queues.read().await.front(&k).await.cloned() };
            if let Some(track) = track {
                if track.handle_uuid == handle.uuid().to_string() {
                    if state.playing == PlayMode::End || state.playing == PlayMode::Stop {
                        self.queues.write().await.pop(&k).await;
                    }
                }
            }
        }
        None
    }
}
