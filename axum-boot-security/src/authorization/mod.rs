use std::{marker::PhantomData, sync::Arc};

use axum::{extract::FromRequestParts, http::request::Parts};
use axum_boot_core::{error::Error, string};
use reqwest::StatusCode;
use tuple::TupleAuthorize;

use crate::user::role::UserRoles;

pub mod jwt;
pub mod tuple;

pub struct HandlerAuthorization<T: TupleAuthorize> {
  phantom: PhantomData<T>,
}

impl<T, S> FromRequestParts<S> for HandlerAuthorization<T>
where
  T: TupleAuthorize,
{
  type Rejection = (StatusCode, String);

  fn from_request_parts(
    parts: &mut Parts,
    _state: &S,
  ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
    async move {
      let user_roles = parts
        .extensions
        .get::<Arc<UserRoles>>()
        .ok_or((StatusCode::FORBIDDEN, string!()))?;
      if !T::authorize(parts, user_roles.as_ref()).map_err(|err| match err {
        Error::Message(message) => (StatusCode::UNAUTHORIZED, message),
        Error::Empty => (StatusCode::UNAUTHORIZED, string!()),
      })? {
        return Err((StatusCode::UNAUTHORIZED, string!()));
      }
      Ok(Self {
        phantom: PhantomData,
      })
    }
  }
}

pub trait HandlerAuthorizer {
  fn authorize(parts: &Parts, user_roles: &UserRoles) -> Result<bool, Error>;
}
