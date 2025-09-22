use std::error::Error;

use axum::http::StatusCode;

pub fn internal_server_error(error: impl Error) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, format!("{error}"))
}
