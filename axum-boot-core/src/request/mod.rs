use axum::{body::Body, http::Request};

pub trait FromRequestRef {
  fn from_reqest_ref(request_ref: &Request<Body>);
}
