use axum::http::request::Parts;
use axum_boot_core::{error::Error, request::FromRequestPartsRef, string};
use jsonwebtoken::{DecodingKey, Validation, decode, decode_header};
use log::error;
use serde::Deserialize;

use crate::{
  oauth2::jwks::Jwks, request::extract::authorization::BearerToken, user::role::UserRoles,
};

use super::HandlerAuthorizer;

pub struct Jwt;

#[derive(Deserialize)]
struct Payload {
  #[serde(rename = "sub")]
  pub _sub: String,
}

impl HandlerAuthorizer for Jwt {
  fn authorize(parts: &Parts, _user_roles: &UserRoles) -> Result<bool, Error> {
    let jwks = parts.extensions.get::<Jwks>().ok_or_else(|| {
      log::error!("Failed to get jwks. Try to add Jwks using extensions");
      Error::Empty
    })?;
    let BearerToken { token } = BearerToken::from_request_parts_ref(parts).map_err(|_| {
      log::error!("Failed to read bearer token");
      Error::Empty
    })?;
    let header = decode_header(&token).map_err(|_| {
      log::error!("Failed to decode jwt header");
      Error::Empty
    })?;
    let jwk = jwks.sig(header).ok_or_else(|| {
      log::error!("Failed to get jwk for sig");
      Error::Empty
    })?;
    let rsa = DecodingKey::from_rsa_components(&jwk.n, &jwk.e).map_err(|_| {
      log::error!("Failed to create decoding key");
      Error::Empty
    })?;
    let validation = parts.extensions.get::<Validation>().ok_or_else(|| {
      log::error!("Failed to get validation. Try to add validation using extensions");
      Error::Empty
    })?;
    let _decoded = decode::<Payload>(&token, &rsa, &validation).map_err(|err| {
      log::debug!("Failed to decode jwt token: {:?}", err);
      Error::Empty
    })?;

    Ok(true)
  }
}
