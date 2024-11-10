use reqwest::Client as HttpClient;

#[derive(Debug)]
pub struct Data {
    pub hc: HttpClient,
}
