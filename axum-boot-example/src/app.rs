use std::{sync::Arc, vec};

use anyhow::Ok;
use axum::{Extension, Router};
use axum_boot_core::util::if_else;
use axum_boot_security::user::{UserService, layer::UserLayer};

use crate::example;

#[derive(Clone, Debug)]
pub struct AppState {
  pub name: String,
}

pub fn app() -> Router {
  Router::new()
    .merge(example::handler::router())
    .layer(user_layer())
    .layer(Extension(AppState {
      name: "test".to_string(),
    }))
}

pub fn user_layer<S>() -> UserLayer<S> {
  UserLayer::from_fn(|req| {
    let include_user = req
      .headers()
      .get("Include-User")
      .map(|hv| hv.to_str().unwrap_or("false"))
      .map(|s| s.parse::<bool>().unwrap_or(false))
      .unwrap_or(false);
    if include_user {
      Some(vec!["user".to_string(), "admin".to_string()])
    } else {
      None
    }
  })
}
