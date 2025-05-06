use std::sync::Arc;

use axum::{
  Router,
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

pub fn router() -> Router {
  Router::new().route("/example", get(get_example))
}

#[function_authorizer]
async fn Check(parts: &Parts) -> bool {
  let roles = UserRoles::from_parts(parts);
  println!("Roles: {:?}", roles.0);
  roles.has_role("user")
}

#[authorize_with(Check)]
async fn get_example() -> AuthorizedResponse<impl IntoResponse> {
  Ok("hello")
}
