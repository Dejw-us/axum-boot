use anyhow::Ok;
use app::app;
use tokio::net::TcpListener;

mod app;
mod example;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let listener = TcpListener::bind("0.0.0.0:8080").await?;

  axum::serve(listener, app()).await?;

  Ok(())
}
