#![allow(unused_imports)]

mod db;
mod cli;
mod error;
mod slack;
mod http;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let _ = dotenvy::dotenv();

  tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .with_test_writer()
    .init();

  let args = cli::parse();
  let db_pool = db::create_pool(&args).await?;
  http::start(&args, db_pool).await?;
  Ok(())
}
