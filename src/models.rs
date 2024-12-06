use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Song {
    pub id: u64,
    pub title: String,
    pub artist: String,
    pub genre: String,
    pub play_count: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NewSong {
    pub title: String,
    pub artist: String,
    pub genre: String,
}