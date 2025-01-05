use actix_web::{web, HttpResponse, Responder};
use crate::state::SharedState;
use crate::models::{Song, NewSong};
use serde_json::json;
use std::collections::HashMap;

pub async fn welcome() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the Rust-powered web server!")
}

pub async fn get_visit_count(state: web::Data<SharedState>) -> impl Responder {
    let mut visit_count = state.visit_count.write();
    *visit_count += 1;
    state.save(); // saves the updated visit count
    HttpResponse::Ok().body(format!("Visit count: {}", *visit_count))
}

pub async fn add_song(state: web::Data<SharedState>, song: web::Json<NewSong>) -> impl Responder {
    let mut next_id = state.next_id.write(); // locks to generate the next unique ID
    let id = *next_id + 1;

    let new_song = Song {
        id,
        title: song.title.clone(),
        artist: song.artist.clone(),
        genre: song.genre.clone(),
        play_count: 0,
    };

    state.songs.insert(id, new_song.clone()); // DashMap insert
    *next_id = id;
    state.save(); // saves the updated state after adding a new song

    HttpResponse::Ok().json(new_song)
}

pub async fn search_songs(state: web::Data<SharedState>, query: web::Query<HashMap<String, String>>) -> impl Responder {
    let results: Vec<Song> = state
        .songs
        .iter()
        .filter_map(|entry| {
            let song = entry.value();
            if query.iter().all(|(key, value)| {
                match key.as_str() {
                    "title" => song.title.to_lowercase().contains(&value.to_lowercase()),
                    "artist" => song.artist.to_lowercase().contains(&value.to_lowercase()),
                    "genre" => song.genre.to_lowercase().contains(&value.to_lowercase()),
                    _ => false,
                }
            }) {
                Some(song.clone()) // clones for the result
            } else {
                None
            }
        })
        .collect();

    HttpResponse::Ok().json(results)
}

pub async fn play_song(state: web::Data<SharedState>, song_id: web::Path<u64>) -> impl Responder {
    let song_id = song_id.into_inner();

    if let Some((_, mut song)) = state.songs.remove(&song_id) {
        song.play_count += 1;
        state.songs.insert(song_id, song.clone()); // re-inserts the updated song back into the DashMap
        state.save();
        HttpResponse::Ok().json(song)
    } else {
        HttpResponse::NotFound().json(json!({ "error": "Song not found" }))
    }
}
