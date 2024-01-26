use sea_orm::{Database, DatabaseConnection};
use migration::{Migrator, MigratorTrait};
use anyhow::Result;

pub mod models;

pub type DbPool = DatabaseConnection;

/// Creates the shared postgres database pool.
///
/// # Parameters
/// - `args`: The command line arguments.
///
/// # Returns
/// Returns a `Result<DbPool>` containing the database pool if successful.
pub async fn create_pool(args: &crate::cli::Args) -> Result<DbPool> {
  let database_url = if args.db_user.is_none() || args.db_pass.is_none() {
    tracing::debug!("Connecting to database without authentication");
    format!(
      "postgres://{}:{}/{}",
      args.db_host,
      args.db_port,
      args.db_name
    )
  } else {
    tracing::debug!("Connecting to database with authentication");
    format!(
      "postgres://{}:{}@{}:{}/{}",
      args.db_user.as_ref().unwrap(),
      args.db_pass.as_ref().unwrap(),
      args.db_host,
      args.db_port,
      args.db_name
    )
  };

  let conn = Database::connect(&database_url).await?;
  tracing::debug!("Running migrations...");
  Migrator::up(&conn, None).await?;
  Ok(conn)
}
