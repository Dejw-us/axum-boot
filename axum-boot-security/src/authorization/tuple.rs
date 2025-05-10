use axum::http::request::Parts;
use axum_boot_core::error::Error;

use crate::user::role::UserRoles;

use super::HandlerAuthorizer;

pub trait TupleAuthorize {
  fn authorize(parts: &Parts, user_roles: &UserRoles) -> Result<bool, Error>;
}

impl TupleAuthorize for () {
  fn authorize(_parts: &Parts, _user_roles: &UserRoles) -> Result<bool, Error> {
    Ok(true)
  }
}

impl<A: HandlerAuthorizer> TupleAuthorize for (A,) {
  fn authorize(parts: &Parts, user_roles: &UserRoles) -> Result<bool, Error> {
    A::authorize(parts, user_roles)
  }
}

impl<A: HandlerAuthorizer, B: HandlerAuthorizer> TupleAuthorize for (A, B) {
  fn authorize(parts: &Parts, user_roles: &UserRoles) -> Result<bool, Error> {
    Ok(A::authorize(parts, user_roles)? && B::authorize(parts, user_roles)?)
  }
}

impl<A: HandlerAuthorizer, B: HandlerAuthorizer, C: HandlerAuthorizer> TupleAuthorize
  for (A, B, C)
{
  fn authorize(parts: &Parts, user_roles: &UserRoles) -> Result<bool, Error> {
    Ok(
      A::authorize(parts, user_roles)?
        && B::authorize(parts, user_roles)?
        && C::authorize(parts, user_roles)?,
    )
  }
}

impl<A: HandlerAuthorizer, B: HandlerAuthorizer, C: HandlerAuthorizer, D: HandlerAuthorizer>
  TupleAuthorize for (A, B, C, D)
{
  fn authorize(parts: &Parts, user_roles: &UserRoles) -> Result<bool, Error> {
    Ok(
      A::authorize(parts, user_roles)?
        && B::authorize(parts, user_roles)?
        && C::authorize(parts, user_roles)?
        && D::authorize(parts, user_roles)?,
    )
  }
}
