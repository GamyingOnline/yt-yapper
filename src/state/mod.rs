use reqwest::Client as HttpClient;
use tokio::sync::mpsc::Sender;

use crate::{persistence::SqlConn, queue::QueueMessage};

#[derive(Debug, Clone, Default)]
pub struct Track {
    pub name: String,
    pub handle_uuid: String,
    pub artist: String,
    pub duration: String,
    pub thumbnail: String,
    pub album: String,
    pub can_scrobble: bool,
    pub from_playlist: bool,
}

#[derive(Debug)]
pub struct Data {
    pub hc: HttpClient,
    pub queue: Sender<QueueMessage>,
    pub sql_conn: SqlConn,
}
