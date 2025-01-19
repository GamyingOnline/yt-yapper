use tokio::sync::mpsc::Sender;

use ::serenity::async_trait;
use serenity::all::GuildId;
use songbird::{
    events::{Event, EventContext, EventHandler as VoiceEventHandler},
    tracks::PlayMode,
};

use crate::{
    queue::{EventState, MusicQueueKey, QueueMessage},
    state::Track,
};
pub struct TrackErrorNotifier {
    pub queues: Sender<QueueMessage>,
    pub guild_id: GuildId,
    pub event_state: EventState,
}

#[async_trait]
impl VoiceEventHandler for TrackErrorNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        let key = MusicQueueKey {
            guild_id: self.guild_id,
            channel_id: self.event_state.channel_id,
        };
        if let EventContext::Track(track_list) = ctx {
            let state = track_list.first();
            if let None = state {
                return None;
            }
            let (state, handle) = state.unwrap();

            let (responder, response) = tokio::sync::oneshot::channel::<Option<Track>>();
            self.queues
                .send(QueueMessage::Front { key, responder })
                .await
                .unwrap();
            if let Ok(Some(track)) = response.await {
                if track.handle_uuid == handle.uuid().to_string() {
                    if state.playing == PlayMode::End || state.playing == PlayMode::Stop {
                        let (responder, response) =
                            tokio::sync::oneshot::channel::<Option<Track>>();
                        self.queues
                            .send(QueueMessage::Pop {
                                key,
                                responder,
                                event_state: self.event_state.clone(),
                            })
                            .await
                            .unwrap();
                        let _ = response.await;
                    }
                }
            }
        }
        None
    }
}
