use spotify_rs::{
    model::search::{Item, SearchResults},
    ClientCredsClient, ClientCredsFlow, Error,
};

#[derive(Debug)]
pub struct SpotifyClient {
    client_id: String,
    client_secret: String,
}

impl SpotifyClient {
    pub fn new(client_id: String, client_secret: String) -> Self {
        Self {
            client_id,
            client_secret,
        }
    }

    pub async fn get_track(&mut self, query: String) -> Result<SearchResults, Error> {
        let mut client = ClientCredsClient::authenticate(ClientCredsFlow::new(
            self.client_id.clone(),
            self.client_secret.clone(),
        ))
        .await?;
        client.search(query, &[Item::Track]).limit(10).get().await
    }
}
