use actix_web::{App, HttpServer, web, middleware::Logger};
use sea_orm::{Database, DatabaseConnection};
use std::sync::Arc;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct AppState {
  pub db: DatabaseConnection
}

pub async fn start(args: &crate::cli::Args, db: DatabaseConnection) -> Result<()> {
  let state = AppState { db };

  let address = format!("{}:{}", args.http_host, args.http_port);
  tracing::info!("Starting server on {address}");
  HttpServer::new(move || {
    App::new()
      .wrap(Logger::default())
      .app_data(web::Data::new(state.clone()))
      .configure(crate::slack::http::config)
  })
  .bind(&address)?
  .run()
  .await?;

  Ok(())
}
