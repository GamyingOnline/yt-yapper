use std::collections::{HashMap, VecDeque};

use async_trait::async_trait;
use serenity::all::{ChannelId, GuildId};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct EventfulQueueKey {
    pub guild_id: GuildId,
    pub channel_id: ChannelId,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum QueueEvents<'a, T> {
    TrackPushed(EventfulQueueKey, &'a VecDeque<T>),
    TrackPopped(EventfulQueueKey, &'a VecDeque<T>),
    QueueCreated(EventfulQueueKey),
    QueueCleared(EventfulQueueKey),
}

#[async_trait]
pub trait QueueEventHandler<T>: std::fmt::Debug
where
    T: Send + Sync,
{
    async fn on_event(&self, event: &QueueEvents<T>);
}

#[derive(Debug, Default)]
pub struct EventfulQueue<T> {
    data: HashMap<EventfulQueueKey, VecDeque<T>>,
    handlers: HashMap<EventfulQueueKey, Box<dyn QueueEventHandler<T> + Send + Sync>>,
}

impl<T: Send + Sync + Clone> EventfulQueue<T> {
    pub fn add_handler<H>(&mut self, handler: H, key: &EventfulQueueKey)
    where
        H: QueueEventHandler<T> + Send + Sync + 'static,
    {
        self.handlers.insert(key.clone(), Box::new(handler));
    }

    pub async fn add_queue(&mut self, key: EventfulQueueKey) {
        let queue = self.data.get(&key);
        if queue.is_some() {
            return;
        }
        self.data.insert(key.clone(), VecDeque::new());
        self.handlers
            .get(&key)
            .unwrap()
            .on_event(&QueueEvents::QueueCreated(key))
            .await;
    }

    pub async fn push(&mut self, key: &EventfulQueueKey, val: T) {
        let queue = self.data.entry(key.clone()).or_insert_with(VecDeque::new);

        queue.push_back(val);

        if let Some(_) = queue.back() {
            self.handlers
                .get(&key)
                .unwrap()
                .on_event(&QueueEvents::TrackPushed(
                    key.clone(),
                    self.data.get(&key).unwrap(),
                ))
                .await;
        }
    }

    pub async fn get_queue(&self, key: &EventfulQueueKey) -> Option<&VecDeque<T>> {
        self.data.get(key)
    }

    pub async fn clear(&mut self, key: EventfulQueueKey) {
        self.data.get_mut(&key).unwrap().clear();
        self.handlers
            .get(&key)
            .unwrap()
            .on_event(&QueueEvents::QueueCleared(key))
            .await;
    }

    pub async fn pop(&mut self, key: &EventfulQueueKey) -> Option<T> {
        let track = self.data.get_mut(&key)?.pop_front();

        if let Some(_) = track {
            self.handlers
                .get(&key)
                .unwrap()
                .on_event(&QueueEvents::TrackPopped(
                    key.clone(),
                    self.data.get(&key).unwrap(),
                ))
                .await;
        }
        track
    }

    pub async fn remove(&mut self, key: EventfulQueueKey, idx: usize) -> Option<T> {
        self.data.get_mut(&key)?.remove(idx)
    }

    pub async fn front(&self, key: &EventfulQueueKey) -> Option<&T> {
        self.data.get(&key)?.front()
    }

    pub async fn key_exists(&mut self, key: &EventfulQueueKey) -> bool {
        self.data.contains_key(key)
    }
}
