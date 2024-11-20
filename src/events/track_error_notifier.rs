use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
};

use ::serenity::async_trait;
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
            if state.playing == PlayMode::End || state.playing == PlayMode::Stop {
                let track = { self.queues.read().await.get(&k).unwrap().front().cloned() };

                if let Some(track) = track {
                    if track.handle_uuid == handle.uuid().to_string() {
                        {
                            self.queues.write().await.get_mut(&k).unwrap().pop_front();
                        }
                    }
                }
                return None;
            }
        }
        None
    }
}
