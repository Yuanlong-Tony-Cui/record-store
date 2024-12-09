use actix_web::{web, HttpResponse, Responder};
use crate::state::SharedState;
use crate::models::{Song, NewSong};
use serde_json::json;
use std::collections::HashMap;

/*
    We use `actix_web` to specify which handler to use for each route.
*/

// "/", GET
pub async fn welcome() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the Rust-powered web server!")
}

// "/count", GET
pub async fn get_visit_count(state: web::Data<SharedState>) -> impl Responder {
    let mut visit_count = state.visit_count.write();
    *visit_count += 1;
    HttpResponse::Ok().body(format!("Visit count: {}", *visit_count))
}

// "/songs/new", POST
pub async fn add_song(state: web::Data<SharedState>, song: web::Json<NewSong>) -> impl Responder {
    let mut next_id = state.next_id.write();
    let mut songs = state.songs.write();
    let id = *next_id + 1;

    let new_song = Song {
        id,
        title: song.title.clone(),
        artist: song.artist.clone(),
        genre: song.genre.clone(),
        play_count: 0,
    };

    songs.insert(id, new_song.clone());
    *next_id = id;

    HttpResponse::Ok().json(new_song)
}

// "/songs/search", GET
pub async fn search_songs(state: web::Data<SharedState>, query: web::Query<HashMap<String, String>>) -> impl Responder {
    let songs = state.songs.read();
    let results: Vec<&Song> = songs
        .values()
        .filter(|song| {
            query.iter().all(|(key, value)| {
                match key.as_str() {
                    "title" => song.title.to_lowercase().contains(&value.to_lowercase()),
                    "artist" => song.artist.to_lowercase().contains(&value.to_lowercase()),
                    "genre" => song.genre.to_lowercase().contains(&value.to_lowercase()),
                    _ => false,
                }
            })
        })
        .collect();

    HttpResponse::Ok().json(results)
}

// "/songs/play/{id}", GET
pub async fn play_song(state: web::Data<SharedState>, song_id: web::Path<u64>) -> impl Responder {
    let mut songs = state.songs.write();

    if let Some(song) = songs.get_mut(&song_id.into_inner()) {
        song.play_count += 1;
        HttpResponse::Ok().json(song)
    } else {
        HttpResponse::NotFound().json(json!({ "error": "Song not found" }))
    }
}
