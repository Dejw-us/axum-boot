use std::sync::Arc;

use axum::{
  extract::FromRequestParts,
  http::{StatusCode, request::Parts},
};

use super::role::UserRoles;

pub struct UserRolesExtractor(pub Arc<UserRoles>);

impl<S> FromRequestParts<S> for UserRolesExtractor {
  type Rejection = StatusCode;

  fn from_request_parts(
    parts: &mut Parts,
    _state: &S,
  ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
    async move {
      match parts.extensions.get::<Arc<UserRoles>>() {
        Some(roles) => Ok(UserRolesExtractor(roles.clone())),
        None => Err(StatusCode::UNAUTHORIZED),
      }
    }
  }
}
