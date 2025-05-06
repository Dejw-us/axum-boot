use std::{marker::PhantomData, sync::Arc};

use axum::{body::Body, http::Request};
use tower::Layer;

use super::UserService;

#[derive(Clone)]
pub struct UserLayer<S> {
  roles_extractor: Arc<dyn Fn(&Request<Body>) -> Option<Vec<String>> + Send + Sync + 'static>,
  _phantom: PhantomData<S>,
}

impl<S> UserLayer<S> {
  pub fn from_fn(
    fn_: impl Fn(&Request<Body>) -> Option<Vec<String>> + Send + Sync + 'static,
  ) -> Self {
    Self::new(Arc::new(fn_))
  }

  pub fn new(
    roles_extractor: Arc<dyn Fn(&Request<Body>) -> Option<Vec<String>> + Send + Sync + 'static>,
  ) -> Self {
    Self {
      roles_extractor,
      _phantom: PhantomData,
    }
  }
}

impl<S> Layer<S> for UserLayer<S>
where
  S: Clone + Send,
{
  type Service = UserService<S>;

  fn layer(&self, inner: S) -> Self::Service {
    UserService::new(inner, self.roles_extractor.clone())
  }
}
