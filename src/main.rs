use axum::{
  routing::get,
  Router
};
use tracing::{info, instrument};

mod cli;
mod error;
mod slack;

#[tokio::main]
async fn main() -> error::Result<()> {
  tracing_subscriber::fmt::init();

  let args = cli::parse();

  let app = Router::new()
    .route("/", get(root));

  info!("Starting server: http://localhost:3000");
  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
  Ok(axum::serve(listener, app).await?)
}

#[instrument]
async fn root() -> &'static str {
  "This is the song that never ends, it goes on and on my friends..."
}
