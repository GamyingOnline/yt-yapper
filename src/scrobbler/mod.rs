use rustfm_scrobble::{Scrobble, Scrobbler};
use sqlx::SqlitePool;

pub async fn scrobble(song: Scrobble, users: Vec<u64>, sql_conn: &SqlitePool) {
    for user in users {
        let song_clone = song.clone();
        let sql_conn_clone = sql_conn.clone();
        tokio::spawn(async move {
            scrobble_for_user(&song_clone, user as i64, &sql_conn_clone).await;
        });
    }
}

pub async fn scrobble_for_user(song: &Scrobble, user: i64, sql_conn: &SqlitePool) {
    let api_key = std::env::var("LASTFM_API_KEY").expect("missing LASTFM_API_KEY");
    let api_secret = std::env::var("LASTFM_API_SECRET").expect("missing LASTFM_API_SECRET");
    let mut scrobbler = Scrobbler::new(&api_key, &api_secret);

    let token = sqlx::query!("SELECT token FROM user WHERE id=(?)", user)
        .fetch_one(sql_conn)
        .await
        .expect("Token not found")
        .token;

    scrobbler.authenticate_with_session_key(&token);

    scrobbler.scrobble(song).expect("Scrobble failed");
}

pub async fn now_playing(song: Scrobble, users: Vec<u64>, sql_conn: &SqlitePool) {
    for user in users {
        let song_clone = song.clone();
        let sql_conn_clone = sql_conn.clone();
        tokio::spawn(async move {
            now_playing_for_user(&song_clone, user as i64, &sql_conn_clone).await;
        });
    }
}

pub async fn now_playing_for_user(song: &Scrobble, user: i64, sql_conn: &SqlitePool) {
    let api_key = std::env::var("LASTFM_API_KEY").expect("missing LASTFM_API_KEY");
    let api_secret = std::env::var("LASTFM_API_SECRET").expect("missing LASTFM_API_SECRET");
    let mut scrobbler = Scrobbler::new(&api_key, &api_secret);

    let token = sqlx::query!("SELECT token FROM user WHERE id=(?)", user)
        .fetch_one(sql_conn)
        .await
        .expect("Token not found")
        .token;

    scrobbler.authenticate_with_session_key(&token);

    scrobbler.now_playing(song).expect("Scrobble failed");
}
