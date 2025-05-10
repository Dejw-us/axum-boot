use std::sync::Arc;

use axum::{
  Extension, Router,
  body::Body,
  extract::{FromRequestParts, Request},
  http::request::Parts,
  response::IntoResponse,
  routing::get,
};
use axum_boot_core::{error::Error, request::FromRequestPartsRef, string};
use axum_boot_security::{
  authorization::{HandlerAuthorization, HandlerAuthorizer, jwt::Jwt},
  macros::{authenticated, authorize, authorize_with, roles},
  oauth2::jwks::Jwks,
  request::extract::authorization::{BasicAuth, BearerToken, JwtClaims},
  response::AuthorizedResponse,
  user::role::UserRoles,
};
use jsonwebtoken::{DecodingKey, Validation, decode, decode_header};
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

#[authorize(Jwt)]
async fn get_example(
  JwtClaims(claims): JwtClaims<Payload>,
) -> AuthorizedResponse<impl IntoResponse> {
  println!("auth: {:?}", claims);
  Ok("hello")
}
