use axum::{
  body::Body,
  http::{Request, request::Parts},
};

pub trait FromRequestRef: Sized {
  type Rejection;

  fn from_reqest_ref(request: &Request<Body>) -> Result<Self, Self::Rejection>;
}

pub trait FromRequestPartsRef: Sized {
  type Rejection;

  fn from_request_parts_ref(parts: &Parts) -> Result<Self, Self::Rejection>;
}
