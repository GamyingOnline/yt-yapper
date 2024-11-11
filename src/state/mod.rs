use reqwest::Client as HttpClient;
use sqlx::{SqliteConnection, SqlitePool};

use crate::scrobbler::Scrobbler;

#[derive(Debug)]
pub struct Data {
    pub hc: HttpClient,
    pub sqlite_conn: SqlitePool,
    pub scrobbler: Scrobbler,
}
