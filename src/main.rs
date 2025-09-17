use std::{error::Error, sync::Arc};

use axum::{Router, response::IntoResponse, routing::get};
use axum_boot_core::service::{Service, ServiceAccessor};
use tokio::net::TcpListener;

#[derive(Clone)]
pub struct AppState {
  pub test_service: Arc<TestService>,
}

impl ServiceAccessor<TestService> for AppState {
  fn get_service(&self) -> Arc<TestService> {
    self.test_service.clone()
  }
}

pub struct TestService;

impl TestService {
  pub fn test(&self) {
    println!("Test");
  }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let listener = TcpListener::bind("0.0.0.0:8080").await?;

  let state = Arc::new(AppState {
    test_service: Arc::new(TestService),
  });

  let app = Router::new().route("/test", get(test)).with_state(state);

  axum::serve(listener, app).await?;

  Ok(())
}

pub async fn test(test_service: Service<TestService>) -> impl IntoResponse {
  test_service.test();
  "test"
}
