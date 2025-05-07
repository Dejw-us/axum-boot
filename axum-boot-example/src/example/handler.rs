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
  macros::{authorize_with, function_authorizer, roles},
  response::AuthorizedResponse,
  user::role::UserRoles,
};

use crate::app::{self, AppState};

pub fn router() -> Router {
  Router::new().route("/example", get(get_example))
}

#[function_authorizer]
async fn check(parts: &Parts) -> bool {
  let roles = UserRoles::from_parts(parts);
  let app_state = parts.extensions.get::<AppState>().unwrap();
  println!("state: {:?}", app_state);
  roles.has_role("user")
}

#[authorize_with(check)]
async fn get_example() -> AuthorizedResponse<impl IntoResponse> {
  Ok("hello")
}
