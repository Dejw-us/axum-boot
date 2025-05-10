use axum::{
  extract::FromRequestParts,
  http::{StatusCode, header::AUTHORIZATION, request::Parts},
};
use axum_boot_core::request::FromRequestPartsRef;
use base64::{Engine, prelude::BASE64_STANDARD};
use jsonwebtoken::{DecodingKey, Header, Validation, decode, decode_header};
use serde::de::DeserializeOwned;

use crate::oauth2::jwks::Jwks;

#[derive(Debug, PartialEq, Eq)]
pub struct BasicAuth {
  pub username: String,
  pub password: String,
}

impl<S> FromRequestParts<S> for BasicAuth {
  type Rejection = StatusCode;

  fn from_request_parts(
    parts: &mut Parts,
    _state: &S,
  ) -> impl std::future::Future<Output = Result<Self, Self::Rejection>> + Send {
    async move {
      parts
        .headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|basic_auth| basic_auth.strip_prefix("Basic "))
        .and_then(|base64| BASE64_STANDARD.decode(base64).ok())
        .and_then(|decoded| String::from_utf8(decoded).ok())
        .and_then(|decoded_str| {
          let mut split = decoded_str.splitn(2, ':');
          Some(BasicAuth {
            username: split.next()?.to_string(),
            password: split.next()?.to_string(),
          })
        })
        .ok_or(StatusCode::UNAUTHORIZED)
    }
  }
}

#[derive(Debug, PartialEq, Eq)]
pub struct BearerToken {
  pub token: String,
}

impl BearerToken {
  pub fn new(token: &str) -> Self {
    Self {
      token: token.to_string(),
    }
  }
}

impl FromRequestPartsRef for BearerToken {
  type Rejection = StatusCode;

  fn from_request_parts_ref(parts_ref: &Parts) -> Result<Self, Self::Rejection> {
    parts_ref
      .headers
      .get(AUTHORIZATION)
      .and_then(|h| h.to_str().ok())
      .and_then(|h| h.strip_prefix("Bearer "))
      .and_then(|token| Some(Self::new(token)))
      .ok_or(StatusCode::UNAUTHORIZED)
  }
}

impl<S> FromRequestParts<S> for BearerToken {
  type Rejection = StatusCode;

  fn from_request_parts(
    parts: &mut Parts,
    _state: &S,
  ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
    async move { Self::from_request_parts_ref(&parts) }
  }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct JwtClaims<T>(pub T)
where
  T: DeserializeOwned + Clone;

impl<S, T> FromRequestParts<S> for JwtClaims<T>
where
  T: DeserializeOwned + Clone,
  S: Sync,
{
  type Rejection = StatusCode;

  fn from_request_parts(
    parts: &mut Parts,
    state: &S,
  ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
    async move {
      let token = BearerToken::from_request_parts(parts, state).await?.token;
      let jwt_split = token.split(".").collect::<Vec<_>>();

      if jwt_split.len() != 3 {
        return Err(StatusCode::UNAUTHORIZED);
      }

      let payload_base64 = jwt_split[1];
      let payload = BASE64_STANDARD
        .decode(payload_base64)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
      let payload = String::from_utf8(payload).map_err(|_| StatusCode::UNAUTHORIZED)?;
      let payload = serde_json::from_str::<T>(&payload).map_err(|_| StatusCode::UNAUTHORIZED)?;
      Ok(Self(payload))
    }
  }
}
