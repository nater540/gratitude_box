#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

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
    .init();

  let args = cli::parse();
  let db_pool = db::create_pool(&args).await?;
  http::start(&args, db_pool).await?;
  Ok(())
}
