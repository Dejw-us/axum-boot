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
  request::extract::authorization::{BasicAuth, BearerToken, JwtClaims},
  response::AuthorizedResponse,
  user::role::UserRoles,
};
use serde::{Deserialize, Serialize};

use crate::app::{self, AppState};

use super::authorizers::check;

pub fn router() -> Router {
  Router::new().route("/example", get(get_example))
}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct Payload {
  exp: u64,
}

#[authorize_with(check)]
async fn get_example(
  JwtClaims(claims): JwtClaims<Payload>,
) -> AuthorizedResponse<impl IntoResponse> {
  println!("auth: {:?}", claims);
  Ok("hello")
}
