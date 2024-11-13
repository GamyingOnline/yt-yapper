use std::{clone, sync::Arc};

use anyhow::Result;
use reqwest::Client;
use rustfm_scrobble::{Scrobble, Scrobbler as LastFmScrobbler};
use sqlx::{SqliteConnection, SqlitePool};

use crate::persistence::entities::user::User;

#[derive(Clone, Debug)]
pub struct Scrobbler {
    pub http_client: Client,
    pub sqlite_conn: SqlitePool,
    pub api_key: String,
    pub token: String,
}

impl Scrobbler {
    pub fn start_login(&self, user_id: i64) -> String {
        format!(
            "http://www.last.fm/api/auth/?api_key={}&user_id={}",
            self.api_key, user_id
        )
    }

    pub async fn scrobble(&mut self, users: Vec<i64>, song_name: &str) -> Result<()> {
        let mut conn = self.sqlite_conn.acquire().await?;
        for user in users {
            let mut stream = sqlx::query("SELECT * FROM users WHERE id = ?")
                .bind(user)
                .fetch_one(&self.sqlite_conn)
                .await?;
        }
        Ok(())
    }
}
