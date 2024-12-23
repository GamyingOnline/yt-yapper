use async_trait::async_trait;
use serenity::all::{ChannelId, Colour, Context, CreateEmbed, CreateMessage, GuildId};

use crate::{
    queue::{EventfulQueueKey, QueueEventHandler, QueueEvents},
    state::Track,
};

#[derive(Clone, Debug)]
pub struct QueueEvent {
    pub channel_id: ChannelId,
    pub guild_id: GuildId,
    pub text_channel_id: ChannelId,
    pub context: Context,
}

#[async_trait]
impl QueueEventHandler<Track> for QueueEvent {
    async fn on_event(&self, event: &QueueEvents<Track>) {
        match event {
            QueueEvents::QueueCreated(k) => {
                println!("Queue created with key: {:?}", k);
            }
            QueueEvents::TrackPushed(k, queue) => {
                let key = EventfulQueueKey {
                    guild_id: self.guild_id,
                    channel_id: self.channel_id,
                };
                if key == *k {
                    let len = queue.len();
                    let track = queue.back().unwrap();
                    match len {
                        1 => {
                            let embed = CreateEmbed::new()
                                .title("**⏯️ Now Playing**")
                                .field(
                                    track.artist.to_string(),
                                    format!("{} [{}]", track.name, track.duration),
                                    true,
                                )
                                .image(track.thumbnail.to_string())
                                .color(Colour::from_rgb(0, 255, 0));
                            self.text_channel_id
                                .send_message(&self.context, CreateMessage::new().add_embed(embed))
                                .await
                                .expect("Failed to send message");
                        }
                        v => {
                            let embed = CreateEmbed::new()
                                .title(format!("**✅ Queued at position #{}**", v))
                                .field(
                                    track.artist.to_string(),
                                    format!("{} [{}]", track.name, track.duration),
                                    true,
                                )
                                .thumbnail(track.thumbnail.to_string())
                                .color(Colour::from_rgb(0, 255, 0));
                            self.text_channel_id
                                .send_message(&self.context, CreateMessage::new().add_embed(embed))
                                .await
                                .expect("Failed to send message");
                        }
                    }
                }
            }
            QueueEvents::TrackPopped(k, queue) => {
                let key = EventfulQueueKey {
                    guild_id: self.guild_id,
                    channel_id: self.channel_id,
                };
                if key == *k {
                    let len = queue.len();
                    match len {
                        0 => {
                            let embed = CreateEmbed::new()
                                .title("**✅ Queue Finished**")
                                .color(Colour::from_rgb(0, 255, 0));
                            self.text_channel_id
                                .send_message(&self.context, CreateMessage::new().add_embed(embed))
                                .await
                                .expect("Failed to send message");
                        }
                        _ => {
                            let track = queue.front().unwrap();
                            let embed = CreateEmbed::new()
                                .title("**⏯️ Now Playing**")
                                .field(
                                    track.artist.to_string(),
                                    format!("{} [{}]", track.name, track.duration),
                                    true,
                                )
                                .image(track.thumbnail.to_string())
                                .color(Colour::from_rgb(0, 255, 0));
                            self.text_channel_id
                                .send_message(&self.context, CreateMessage::new().add_embed(embed))
                                .await
                                .expect("Failed to send message");
                        }
                    }
                }
            }
            QueueEvents::QueueCleared(k) => {
                let key = EventfulQueueKey {
                    guild_id: self.guild_id,
                    channel_id: self.channel_id,
                };
                if key == *k {
                    let embed = CreateEmbed::new()
                        .title("⏩ Queue Cleared.")
                        .color(Colour::from_rgb(0, 255, 0));
                    self.text_channel_id
                        .send_message(&self.context, CreateMessage::new().add_embed(embed))
                        .await
                        .expect("Failed to send message.");
                }
            }
        }
    }
}
