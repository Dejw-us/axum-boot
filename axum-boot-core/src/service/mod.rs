use std::{ops::Deref, sync::Arc};

use axum::{extract::FromRequestParts, http::request::Parts};

pub trait ServiceAccessor<T> {
  fn get_service(&self) -> Arc<T>;
}

pub struct Service<T>(Arc<T>);

impl<T> Deref for Service<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    self.0.as_ref()
  }
}

impl<T> Service<T> {
  pub fn new(service: Arc<T>) -> Self {
    Self(service)
  }
}

impl<T, S> FromRequestParts<Arc<S>> for Service<T>
where
  S: ServiceAccessor<T>,
  T: Sync + Send,
{
  type Rejection = ();

  fn from_request_parts(
    _parts: &mut Parts,
    state: &Arc<S>,
  ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
    let service = Service::new(state.get_service());

    async move { Ok(service) }
  }
}
