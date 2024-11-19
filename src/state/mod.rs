use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
};

use reqwest::Client as HttpClient;
use tokio::sync::RwLock;

#[derive(Debug)]
pub struct Data {
    pub hc: HttpClient,
    pub queue: Arc<RwLock<HashMap<String, VecDeque<String>>>>,
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
