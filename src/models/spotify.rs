use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub collaborative: bool,
    #[serde(rename = "external_urls")]
    pub external_urls: ExternalUrls,
    pub followers: Followers,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    #[serde(rename = "primary_color")]
    pub primary_color: Value,
    pub name: String,
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub uri: String,
    pub owner: Owner,
    pub public: bool,
    #[serde(rename = "snapshot_id")]
    pub snapshot_id: String,
    pub tracks: Tracks,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalUrls {
    pub spotify: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Followers {
    pub href: Value,
    pub total: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub url: String,
    pub height: i64,
    pub width: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Owner {
    pub href: String,
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub uri: String,
    #[serde(rename = "display_name")]
    pub display_name: String,
    #[serde(rename = "external_urls")]
    pub external_urls: ExternalUrls2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalUrls2 {
    pub spotify: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tracks {
    pub limit: i64,
    pub next: String,
    pub offset: i64,
    pub previous: Value,
    pub href: String,
    pub total: i64,
    pub items: Vec<Item>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    #[serde(rename = "added_at")]
    pub added_at: String,
    #[serde(rename = "primary_color")]
    pub primary_color: Value,
    #[serde(rename = "video_thumbnail")]
    pub video_thumbnail: VideoThumbnail,
    #[serde(rename = "is_local")]
    pub is_local: bool,
    #[serde(rename = "added_by")]
    pub added_by: AddedBy,
    pub track: Track,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoThumbnail {
    pub url: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddedBy {
    #[serde(rename = "external_urls")]
    pub external_urls: ExternalUrls3,
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub uri: String,
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalUrls3 {
    pub spotify: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Track {
    #[serde(rename = "preview_url")]
    pub preview_url: String,
    #[serde(rename = "available_markets")]
    pub available_markets: Vec<String>,
    pub explicit: bool,
    #[serde(rename = "type")]
    pub type_field: String,
    pub episode: bool,
    pub track: bool,
    pub album: Album,
    pub artists: Vec<Artist2>,
    #[serde(rename = "disc_number")]
    pub disc_number: i64,
    #[serde(rename = "track_number")]
    pub track_number: i64,
    #[serde(rename = "duration_ms")]
    pub duration_ms: i64,
    #[serde(rename = "external_ids")]
    pub external_ids: ExternalIds,
    #[serde(rename = "external_urls")]
    pub external_urls: ExternalUrls7,
    pub href: String,
    pub id: String,
    pub name: String,
    pub popularity: i64,
    pub uri: String,
    #[serde(rename = "is_local")]
    pub is_local: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Album {
    #[serde(rename = "available_markets")]
    pub available_markets: Vec<String>,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "album_type")]
    pub album_type: String,
    pub href: String,
    pub id: String,
    pub images: Vec<Image2>,
    pub name: String,
    #[serde(rename = "release_date")]
    pub release_date: String,
    #[serde(rename = "release_date_precision")]
    pub release_date_precision: String,
    pub uri: String,
    pub artists: Vec<Artist>,
    #[serde(rename = "external_urls")]
    pub external_urls: ExternalUrls5,
    #[serde(rename = "total_tracks")]
    pub total_tracks: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image2 {
    pub url: String,
    pub width: i64,
    pub height: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Artist {
    #[serde(rename = "external_urls")]
    pub external_urls: ExternalUrls4,
    pub href: String,
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub uri: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalUrls4 {
    pub spotify: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalUrls5 {
    pub spotify: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Artist2 {
    #[serde(rename = "external_urls")]
    pub external_urls: ExternalUrls6,
    pub href: String,
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub uri: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalUrls6 {
    pub spotify: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalIds {
    pub isrc: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalUrls7 {
    pub spotify: String,
}
