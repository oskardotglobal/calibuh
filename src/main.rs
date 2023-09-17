mod components;
mod handlers;
mod models;
mod util;

use crate::components::book::BookComponent;
use crate::util::HandlerState;
use axum::routing::get;
use axum::Router;
use std::net::SocketAddr;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let state = HandlerState::new().await;

    let app = Router::new()
        .route("/", get(handlers::root::root))
        .route("/hello", get(handlers::hello::hello))
        .route("/books/:id/comments", get(handlers::comments::get_comment))
        .route("/books/:id/cover.jpg", get(handlers::cover::get_cover))
        .nest_service("/assets", ServeDir::new(state.library_path.clone()))
        .nest_service("/static", ServeDir::new("src/static"))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap_or_else(|_| panic!("Couldn't start server, is {} already in use?", addr));
}
