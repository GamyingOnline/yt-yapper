use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
};

use ::serenity::async_trait;
use songbird::events::{Event, EventContext, EventHandler as VoiceEventHandler};
use tokio::sync::RwLock;
pub struct TrackErrorNotifier {
    pub queues: Arc<RwLock<HashMap<String, VecDeque<String>>>>,
    pub channel_id: u64,
    pub guild_id: u64,
}

#[async_trait]
impl VoiceEventHandler for TrackErrorNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        println!("{:#?}",ctx);
        let k = format!("{},{}", self.guild_id, self.channel_id);
        // if let EventContext::Track(track_list) = ctx {
        //     let state = track_list.first();
        //     if let None = state {
        //         return None;
        //     }
        //     let (state, _) = state.unwrap();
        //     if state.playing == PlayMode::End || state.playing == PlayMode::Stop {
        //         println!("STATE:{:?}", state);
        //         return None;
        //     }
        //     self.queues.write().await.get_mut(&k).unwrap().pop_front();
        // }
        None
    }
}
