#![allow(unused_variables)]

use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseBackend, DatabaseConnection, Statement};
use migration::{Migrator, MigratorTrait};
use std::env;

pub async fn setup_test_db() -> anyhow::Result<DatabaseConnection> {
  dotenvy::dotenv().ok();

  // Connect to the test database
  let database_url = format!("{}/gratitude_box_test", base_url(), db_name);
  let conn = Database::connect(database_url).await?;
  Migrator::up(&conn, None).await?;

  Ok(conn)
}

fn base_url() -> String {
  let db_user = env::var("GB_DB_USER").ok();
  let db_pass = env::var("GB_DB_PASS").ok();
  let db_host = env::var("GB_DB_HOST").unwrap_or_else(|_| "localhost".to_string());
  let db_port = env::var("GB_DB_PORT").unwrap_or_else(|_| "5432".to_string());

  if db_user.is_none() || db_pass.is_none() {
    format!(
      "postgres://{}:{}",
      db_host,
      db_port
    )
  } else {
    format!(
      "postgres://{}:{}@{}:{}",
      db_user.as_ref().unwrap(),
      db_pass.as_ref().unwrap(),
      db_host,
      db_port
    )
  }
}
