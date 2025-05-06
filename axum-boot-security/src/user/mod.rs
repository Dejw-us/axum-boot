use axum::{Extension, body::Body, extract::Request, response::Response};
use futures_util::future::BoxFuture;
use role::UserRoles;
use std::{marker::PhantomData, sync::Arc};
use tower::Service;

pub mod extract;
pub mod layer;
pub mod role;

#[derive(Clone)]
pub struct UserService<S> {
  roles_extractor: Arc<dyn Fn(&Request<Body>) -> Option<Vec<String>> + Send + Sync + 'static>,
  inner: S,
}

impl<S> UserService<S> {
  pub fn new(
    inner: S,
    roles_extractor: Arc<dyn Fn(&Request<Body>) -> Option<Vec<String>> + Send + Sync + 'static>,
  ) -> Self {
    UserService {
      roles_extractor,
      inner,
    }
  }
}

impl<S> UserService<S> {
  pub fn get_user_roles(&self, req: &Request<Body>) -> Option<Vec<String>> {
    (self.roles_extractor)(req)
  }
}

impl<S> Service<Request> for UserService<S>
where
  S: Service<Request, Response = Response> + Send + 'static,
  S::Future: Send + 'static,
{
  type Response = S::Response;
  type Error = S::Error;
  type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

  fn poll_ready(
    &mut self,
    cx: &mut std::task::Context<'_>,
  ) -> std::task::Poll<Result<(), Self::Error>> {
    self.inner.poll_ready(cx)
  }

  fn call(&mut self, mut req: Request) -> Self::Future {
    if let Some(roles) = self.get_user_roles(&req) {
      req.extensions_mut().insert(Arc::new(UserRoles(roles)));
    }
    let future = self.inner.call(req);
    Box::pin(async move { Ok(future.await?) })
  }
}
