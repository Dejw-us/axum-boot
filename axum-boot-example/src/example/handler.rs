use std::sync::Arc;

use axum::{
  Extension, Router,
  body::Body,
  extract::{FromRequestParts, Request},
  http::request::Parts,
  response::IntoResponse,
  routing::get,
};
use axum_boot_security::{
  macros::{authenticated, authorize_with, roles},
  request::extract::BasicAuth,
  response::AuthorizedResponse,
  user::role::UserRoles,
};

use crate::app::{self, AppState};

use super::authorizers::check;

pub fn router() -> Router {
  Router::new().route("/example", get(get_example))
}

#[authorize_with(check)]
#[roles("admin", "user")]
async fn get_example(basic_auth: BasicAuth) -> AuthorizedResponse<impl IntoResponse> {
  println!("auth: {:?}", basic_auth);
  Ok("hello")
}
