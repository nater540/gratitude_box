use axum::{
  routing::{get, post},
  http::StatusCode,
  Json, Router,
};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};

mod slack;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  tracing_subscriber::fmt::init();

  let app = Router::new()
    .route("/", get(root));

  info!("Starting server: http://localhost:3000");
  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
  Ok(axum::serve(listener, app).await?)
}

#[instrument]
async fn root() -> &'static str {
  "Hello, World!"
}
