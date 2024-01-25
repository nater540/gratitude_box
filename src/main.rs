use actix_web::{App, HttpServer, web, middleware::Logger};

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

  tracing::debug!("Starting server on {}:{}", args.http_host, args.http_port);
  Ok(HttpServer::new(move || {
    App::new()
      .wrap(Logger::default())
      .app_data(web::Data::new(db_pool.clone()))
      .configure(slack::http::config)
  })
  .bind((args.http_host, args.http_port))?
  .run()
  .await?)
}
