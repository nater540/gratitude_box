use deadpool_diesel::postgres::{Manager, Pool};
use deadpool_diesel::Runtime;
use anyhow::Result;

pub mod schema;
pub mod models;

pub type DbPool = Pool;

#[cfg(test)]
mod test_helpers;

#[cfg(test)]
use test_helpers::connection as test_connection;

/// Creates the shared postgres database pool.
///
/// # Parameters
/// - `args`: The command line arguments.
///
/// # Returns
/// Returns a `Result<Pool>` containing the database pool if successful.
pub fn create_pool(args: &crate::cli::Args) -> Result<Pool> {
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

  let manager = Manager::new(database_url, Runtime::Tokio1);
  Ok(Pool::builder(manager).max_size(args.db_pool).build()?)
}
