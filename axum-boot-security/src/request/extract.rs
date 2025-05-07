use axum::{
  extract::FromRequestParts,
  http::{StatusCode, header::AUTHORIZATION, request::Parts},
};
use base64::{Engine, engine::general_purpose::STANDARD};

#[derive(Debug)]
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
        .and_then(|base64| STANDARD.decode(base64).ok())
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
