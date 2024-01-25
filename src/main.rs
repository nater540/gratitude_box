use actix_web::{App, HttpServer};

mod db;
mod cli;
mod error;
mod slack;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let _ = dotenvy::dotenv();

  tracing_subscriber::fmt::init();

  let args = cli::parse();

  let db_pool = db::create_pool(&args)?;

  let address = format!("{}:{}", args.http_host, args.http_port);
  tracing::debug!("Starting server on {}", address);

  Ok(HttpServer::new(|| {
    App::new().configure(slack::http::config)
  })
  .bind((args.http_host, args.http_port))?
  .run()
  .await?)
}
