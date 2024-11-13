use std::sync::mpsc::Receiver;

use reqwest::Client as HttpClient;
use sqlx::SqlitePool;

use crate::{persistence::entities::user::User, scrobbler::Scrobbler};

#[derive(Debug)]
pub struct Data {
    pub hc: HttpClient,
    pub sqlite_conn: SqlitePool,
    pub scrobbler: Scrobbler,
    pub rx: Receiver<User>,
}
