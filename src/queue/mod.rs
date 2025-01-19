use std::{
    collections::{HashMap, VecDeque},
    time::Duration,
};

use serenity::all::{ChannelId, Colour, Context, CreateEmbed, CreateMessage, GuildId};
use tokio::{
    sync::{mpsc, oneshot::Sender},
    time::sleep,
};

use crate::{
    commands::utils::{handle_playing, scrobble},
    persistence::SqlConn,
    state::Track,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct MusicQueueKey {
    pub guild_id: GuildId,
    pub channel_id: ChannelId,
}

#[derive(Debug, Clone)]
pub struct EventState {
    pub context: Context,
    pub channel_id: ChannelId,
    pub text_channel_id: ChannelId,
    pub sql_conn: SqlConn,
}

#[derive(Debug)]
pub enum QueueMessage {
    AddQueue {
        key: MusicQueueKey,
    },
    Push {
        key: MusicQueueKey,
        value: Track,
        event_state: EventState,
    },
    Pop {
        key: MusicQueueKey,
        responder: Sender<Option<Track>>,
        event_state: EventState,
    },
    Front {
        key: MusicQueueKey,
        responder: Sender<Option<Track>>,
    },
    Clear {
        key: MusicQueueKey,
        event_state: EventState,
    },
    GetQueue {
        key: MusicQueueKey,
        responder: Sender<Option<VecDeque<Track>>>,
    },
    Remove {
        key: MusicQueueKey,
        index: usize,
        responder: Sender<Option<Track>>,
    },
}

#[derive(Default, Debug)]
pub struct MusicQueue {
    data: HashMap<MusicQueueKey, VecDeque<Track>>,
}

impl MusicQueue {
    pub fn spawn(capacity: usize) -> mpsc::Sender<QueueMessage> {
        let (sender, mut receiver) = mpsc::channel::<QueueMessage>(capacity);

        tokio::spawn(async move {
            let mut state = Self::default();

            while let Some(message) = receiver.recv().await {
                match message {
                    QueueMessage::AddQueue { key } => {
                        state.data.entry(key).or_insert_with(VecDeque::new);
                        println!("Queue created for key: {:?}", key);
                    }
                    QueueMessage::Push {
                        key,
                        value,
                        event_state,
                    } => {
                        if let Some(queue) = state.data.get_mut(&key) {
                            queue.push_back(value.clone());
                            let len = queue.len();
                            println!("Track pushed to key: {:?}", key);
                            match len {
                                1 => {
                                    handle_playing(
                                        event_state.context.clone(),
                                        event_state.text_channel_id,
                                        &value,
                                        event_state.channel_id,
                                        &event_state.sql_conn,
                                    )
                                    .await
                                }
                                v => {
                                    if !value.from_playlist {
                                        let embed = CreateEmbed::new()
                                            .title(format!("**✅ Queued at position #{}**", v))
                                            .field(
                                                value.artist.to_string(),
                                                format!("{} [{}]", value.name, value.duration),
                                                true,
                                            )
                                            .thumbnail(value.thumbnail.to_string())
                                            .color(Colour::from_rgb(0, 255, 0));
                                        event_state
                                            .text_channel_id
                                            .send_message(
                                                &event_state.context,
                                                CreateMessage::new().add_embed(embed),
                                            )
                                            .await
                                            .expect("Failed to send message");
                                    }
                                }
                            };
                        }
                    }
                    QueueMessage::Pop {
                        key,
                        responder,
                        event_state,
                    } => {
                        if let Some(queue) = state.data.get_mut(&key) {
                            let result = queue.pop_front();
                            let _ = responder.send(result.clone());
                            let len = queue.len();
                            match len {
                                0 => {
                                    let embed = CreateEmbed::new()
                                        .title("**✅ Queue Finished**")
                                        .color(Colour::from_rgb(0, 255, 0));
                                    let message = event_state
                                        .text_channel_id
                                        .send_message(
                                            &event_state.context,
                                            CreateMessage::new().add_embed(embed),
                                        )
                                        .await
                                        .expect("Failed to send message");
                                    sleep(Duration::new(3, 0)).await;
                                    message.delete(&event_state.context).await.ok();
                                }
                                _ => {
                                    let track = queue.front().unwrap();
                                    handle_playing(
                                        event_state.context.clone(),
                                        event_state.text_channel_id,
                                        track,
                                        event_state.channel_id,
                                        &event_state.sql_conn,
                                    )
                                    .await;
                                }
                            }
                            let track = result.unwrap();
                            if track.can_scrobble {
                                scrobble(
                                    event_state.context.clone(),
                                    &track,
                                    event_state.channel_id,
                                    &event_state.sql_conn,
                                )
                                .await;
                            }
                        }
                    }
                    QueueMessage::Front { key, responder } => {
                        let result = state
                            .data
                            .get(&key)
                            .and_then(|queue| queue.front().cloned());
                        let _ = responder.send(result);
                    }
                    QueueMessage::Clear { key, event_state } => {
                        if let Some(queue) = state.data.get_mut(&key) {
                            queue.clear();
                            let embed = CreateEmbed::new()
                                .title("⏩ Queue Cleared.")
                                .color(Colour::from_rgb(0, 255, 0));
                            event_state
                                .text_channel_id
                                .send_message(
                                    &event_state.context,
                                    CreateMessage::new().add_embed(embed),
                                )
                                .await
                                .expect("Failed to send message.");
                            println!("Queue cleared for key: {:?}", key);
                        }
                    }
                    QueueMessage::GetQueue { key, responder } => {
                        let result = state.data.get(&key).cloned();
                        let _ = responder.send(result);
                    }
                    QueueMessage::Remove {
                        key,
                        index,
                        responder,
                    } => {
                        let result = state
                            .data
                            .get_mut(&key)
                            .and_then(|queue| {
                                if index < queue.len() {
                                    Some(queue.remove(index))
                                } else {
                                    None
                                }
                            })
                            .flatten();
                        let _ = responder.send(result);
                    }
                }
            }
        });
        sender
    }
}
