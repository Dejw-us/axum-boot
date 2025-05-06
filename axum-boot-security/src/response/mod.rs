use axum::{http::StatusCode, response::IntoResponse};

pub type AuthorizedResponse<T> = Result<T, StatusCode>;
