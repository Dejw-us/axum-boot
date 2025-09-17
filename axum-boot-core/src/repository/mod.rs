use std::{ops::Deref, sync::Arc};

use axum::{extract::FromRequestParts, http::request::Parts};

pub trait RepositoryAccessor<T> {
  fn get_repository(&self) -> Arc<T>;
}

pub struct Repository<T>(Arc<T>);

impl<T> Deref for Repository<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    self.0.as_ref()
  }
}

impl<T> Repository<T> {
  pub fn new(repository: Arc<T>) -> Self {
    Self(repository)
  }
}

impl<T, S> FromRequestParts<Arc<S>> for Repository<T>
where
  S: RepositoryAccessor<T>,
  T: Sync + Send,
{
  type Rejection = ();

  fn from_request_parts(
    _parts: &mut Parts,
    state: &Arc<S>,
  ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
    let repository = Repository::new(state.get_repository());

    async move { Ok(repository) }
  }
}
