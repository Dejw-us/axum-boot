use anyhow::Ok;
use app::app;
use axum_boot_security::oauth2::jwks::Jwks;
use tokio::net::TcpListener;

mod app;
mod example;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let listener = TcpListener::bind("0.0.0.0:8081").await?;

  axum::serve(listener, app().await?).await?;

  Ok(())
}
