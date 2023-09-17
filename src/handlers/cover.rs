use crate::models::book::Book;
use crate::util::HandlerState;
use axum::extract::{Path, State};
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::IntoResponse;

pub async fn get_cover(
    State(state): State<HandlerState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let book = match Book::get_by_id(&state.pool, id).await {
        Ok(record) => record,
        Err(e) => panic!("Couldn't select book, got error: {}", e),
    };

    if !book.has_cover() {
        return (StatusCode::NOT_FOUND, HeaderMap::new());
    }

    let cover_path = match book.path.strip_prefix("/") {
        Some(path) => format!("/assets/{}/cover.jpg", path),
        None => format!("/assets/{}/cover.jpg", book.path),
    };

    let mut headers = HeaderMap::new();
    headers.insert(header::LOCATION, cover_path.parse().unwrap());

    (StatusCode::PERMANENT_REDIRECT, headers)
}
