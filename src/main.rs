mod handlers;
mod models;
mod state;

use actix_web::{web, App, HttpServer};
use state::{AppState};
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    /*
        We use `Arc` to wrap `AppState`
        to provide shared ownership across threads, enabling safe concurrent access.
        (Each request handler needs access to the shared AppState,
        but Rust's ownership model requires it to be thread-safe and prevent data races.)
    */
    let state = Arc::new(AppState::load());

    println!("The server is currently listening on localhost:8080.");

    let cloned_state = state.clone();
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .route("/", web::get().to(handlers::welcome))
            .route("/count", web::get().to(handlers::get_visit_count))
            .route("/songs/new", web::post().to(handlers::add_song))
            .route("/songs/search", web::get().to(handlers::search_songs))
            .route("/songs/play/{id}", web::get().to(handlers::play_song))
    })
    .bind("127.0.0.1:8080")?
    .run();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("Failed to install CTRL+C signal handler");
        cloned_state.save();
    });

    server.await
}
