use crate::models::Song;
use parking_lot::RwLock;
use serde_json;
use std::collections::HashMap;
use std::fs::{File};
use std::io::{Read, Write};
use std::sync::Arc;

#[derive(Default)]
pub struct AppState {
    pub visit_count: RwLock<u64>,
    pub songs: RwLock<HashMap<u64, Song>>,
    pub next_id: RwLock<u64>,
}

impl AppState {
    const FILE_PATH: &'static str = "library.json";

    // Load the state from the file
    pub fn load() -> Self {
        match File::open(Self::FILE_PATH) {
            Ok(mut file) => {
                let mut contents = String::new();
                if file.read_to_string(&mut contents).is_ok() {
                    if let Ok(data) = serde_json::from_str::<HashMap<u64, Song>>(&contents) {
                        let next_id = data.keys().max().unwrap_or(&0) + 1;
                        return Self {
                            songs: RwLock::new(data),
                            next_id: RwLock::new(next_id),
                            visit_count: RwLock::new(0),
                        };
                    }
                }
            }
            Err(_) => {}
        }
        Self::default()
    }

    /// Save the state to the file
    pub fn save(&self) {
        if let Ok(mut file) = File::create(Self::FILE_PATH) {
            let songs = self.songs.read();
            if let Ok(data) = serde_json::to_string(&*songs) {
                let _ = file.write_all(data.as_bytes());
            }
        }
    }
}

pub type SharedState = Arc<AppState>;
