use std::sync::Arc;

use reqwest::Client as HttpClient;
use tokio::sync::RwLock;

use crate::queue::EventfulQueue;

#[derive(Debug, Clone, Default)]
pub struct Track {
    pub name: String,
    pub handle_uuid: String,
    pub artist: String,
    pub duration: String,
    pub thumbnail: String,
}

#[derive(Debug)]
pub struct Data {
    pub hc: HttpClient,
    pub queue: Arc<RwLock<EventfulQueue<Track>>>,
}

// pub struct Track {
//     pub name: String,
//     pub is_playing: bool
// }
// impl Track {
//     pub fn new_from_name(name: impl Into<String>) -> Self {
//         Self { name: name.into(), is_playing: false }
//     }
//     pub fn new(name: impl Into<String>, is_playing: bool) -> Self {
//         Self { name: name.into(), is_playing }
//     }
// }
