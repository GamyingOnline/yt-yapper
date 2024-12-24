use anyhow::Error;
use rustfm_scrobble::{Scrobble, Scrobbler as RustFmScrobbler};

use crate::persistence::entities::user::User;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Scrobbler {
    api_key: String,
    api_secret: String,
}

impl Scrobbler {
    pub fn new(api_key: String, api_secret: String) -> Self {
        Scrobbler {
            api_key,
            api_secret,
        }
    }

    pub async fn track_to_scrobble(
        &self,
        artist: &String,
        track: &String,
        album: &String,
    ) -> Scrobble {
        Scrobble::new(artist, track, album)
    }

    pub async fn scrobble(&mut self, song: &Scrobble, user: User) {
        let mut scrobbler = RustFmScrobbler::new(&self.api_key, &self.api_secret);
        scrobbler.authenticate_with_session_key(&user.token);
        scrobbler.scrobble(song).expect("Scrobble failed");
    }

    pub async fn now_playing(&mut self, song: &Scrobble, user: User) {
        let mut scrobbler = RustFmScrobbler::new(&self.api_key, &self.api_secret);
        scrobbler.authenticate_with_session_key(&user.token);
        scrobbler.scrobble(song).expect("Now Playing failed");
    }

    pub async fn get_user_token(
        &mut self,
        username: &String,
        password: &String,
    ) -> Result<String, Error> {
        let mut scrobbler = RustFmScrobbler::new(&self.api_key, &self.api_secret);
        let res = scrobbler
            .authenticate_with_password(username, password)
            .expect("Invalid creds");
        Ok(res.key)
    }
}
