use dashmap::DashMap;
use crate::models::Song;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use parking_lot::RwLock;
use std::sync::Arc;

#[derive(Default)]
pub struct AppState {
    pub visit_count: RwLock<u64>, // atomic counter
    pub songs: DashMap<u64, Song>, // concurrent map
    pub next_id: RwLock<u64>, // atomic counter
}

impl AppState {
    // ASSUMPTION: <music-library.json> never gets added, modified, or deleted manually.
    const FILE_PATH: &'static str = "music-library.json";

    /// Load the state from the file
    pub fn load() -> Self {
        match File::open(Self::FILE_PATH) {
            Ok(mut file) => {
                let mut contents = String::new();
                if file.read_to_string(&mut contents).is_ok() {
                    if let Ok(data) = serde_json::from_str::<HashMap<u64, Song>>(&contents) {
                        let dashmap = DashMap::new();
                        for (key, value) in data {
                            dashmap.insert(key, value);
                        }

                        let next_id = dashmap.iter().map(|entry| *entry.key()).max().unwrap_or(0) + 1;
                        return Self {
                            songs: dashmap,
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
            let hash_map: HashMap<u64, Song> = self
                .songs
                .iter()
                .map(|entry| (*entry.key(), entry.value().clone()))
                .collect();
            if let Ok(data) = serde_json::to_string(&hash_map) {
                let _ = file.write_all(data.as_bytes());
            }
        }
    }
}

pub type SharedState = Arc<AppState>;
