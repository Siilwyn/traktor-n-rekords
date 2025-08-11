use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename = "NML")]
#[serde(rename_all = "UPPERCASE")]
pub struct Nml {
    #[serde(rename = "@VERSION")]
    pub version: String,
    pub collection: Collection,
}

#[derive(Serialize, Deserialize)]
pub struct Collection {
    #[serde(rename = "ENTRY")]
    pub entries: Option<Vec<Entry>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct Entry {
    #[serde(rename = "@TITLE")]
    pub title: String,
    #[serde(rename = "@ARTIST", default = "empty_string")]
    pub artist: String,
    pub location: Location,
    pub info: Info,
    pub tempo: Tempo,
    #[serde(rename = "CUE_V2")]
    pub cues: Option<Vec<CueV2>>,
}

#[derive(Serialize, Deserialize)]
pub struct Location {
    #[serde(rename = "@DIR")]
    pub dir: String,
    #[serde(rename = "@FILE")]
    pub file: String,
    #[serde(rename = "@VOLUME")]
    pub volume: String,
}

#[derive(Serialize, Deserialize)]
pub struct Info {
    #[serde(rename = "@GENRE", default = "empty_string")]
    pub genre: String,
    #[serde(rename = "@KEY", default = "empty_string")]
    pub key: String,
    #[serde(rename = "@PLAYTIME")]
    pub playtime: String,
}

#[derive(Serialize, Deserialize)]
pub struct Tempo {
    #[serde(rename = "@BPM")]
    pub bpm: String,
}

#[derive(Serialize, Deserialize)]
pub struct CueV2 {
    #[serde(rename = "@NAME")]
    pub name: String,
    #[serde(rename = "@START")]
    pub start: String,
    #[serde(rename = "@HOTCUE")]
    pub hotcue: i32,
}

fn empty_string() -> String {
    "".to_string()
}
