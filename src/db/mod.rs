use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use migration::{Migrator, MigratorTrait};
use anyhow::Result;

#[cfg(test)]
pub mod test_helpers;

#[cfg(test)]
pub use test_helpers::*;

pub mod entities;

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

  let mut opts = ConnectOptions::new(&database_url);
  opts.min_connections(args.db_pool_min)
      .max_connections(args.db_pool_max);

  let conn = Database::connect(opts).await?;
  tracing::debug!("Running migrations...");
  Migrator::up(&conn, None).await?;
  Ok(conn)
}
