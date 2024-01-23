use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};

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
