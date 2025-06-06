use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename = "DJ_PLAYLISTS")]
#[serde(rename_all = "UPPERCASE")]
pub struct DjPlaylists {
    #[serde(rename = "@Version")]
    pub version: String,
    pub collection: Collection,
}

#[derive(Deserialize, Serialize)]
pub struct Collection {
    #[serde(rename = "TRACK")]
    pub tracks: Option<Vec<Track>>,
}

#[derive(Deserialize, Serialize)]
pub struct Track {
    #[serde(rename = "@Name")]
    pub title: String,
    #[serde(rename = "@Artist")]
    pub artist: String,
    #[serde(rename = "@Genre")]
    pub genre: String,
    #[serde(rename = "@Tonality")]
    pub key: String,
    #[serde(rename = "@Location")]
    pub location: String,
    #[serde(rename = "@AverageBpm")]
    pub bpm: String,
    #[serde(rename = "@TotalTime")]
    pub total_time: String,
    #[serde(rename = "POSITION_MARK")]
    pub position_marks: Option<Vec<PositionMark>>,
}

#[derive(Deserialize, Serialize)]
pub struct PositionMark {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@Type")]
    pub r#type: String,
    #[serde(rename = "@Start")]
    pub start: String,
    #[serde(rename = "@Num")]
    pub num: i32,
}
