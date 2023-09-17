use crate::util::HandlerState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;

#[derive(Debug, PartialEq)]
struct Comment {
    pub text: Option<String>,
}

pub async fn get_comment(
    State(state): State<HandlerState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    sqlx::query_as!(Comment, "select text from comments where book = ?", id)
        .fetch_one(&state.pool)
        .await
        .map(|comment| (StatusCode::OK, comment.text.unwrap_or("".to_string())))
        .unwrap_or((StatusCode::NOT_FOUND, "Comment not found".to_string()))
}
