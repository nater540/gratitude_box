use actix_web::{middleware, App, HttpServer};

mod db;
mod cli;
mod error;
mod slack;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let _ = dotenvy::dotenv();

  tracing_subscriber::fmt::init();

  let args = cli::parse();

  Ok(HttpServer::new(|| {
    App::new().configure(slack::http::config)
  })
  .bind(("127.0.0.1", 3000))?
  .run()
  .await?)
}
