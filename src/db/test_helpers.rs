use diesel::prelude::*;
use diesel_migrations::{FileBasedMigrations, MigrationHarness};

pub fn connection() -> PgConnection {
  dotenvy::dotenv().ok();

  let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  let mut conn = PgConnection::establish(&url).unwrap();
  let migrations = FileBasedMigrations::find_migrations_directory().unwrap();
  conn.run_pending_migrations(migrations).unwrap();
  conn.begin_test_transaction().unwrap();
  conn
}
