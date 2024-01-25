pub use sea_orm_migration::prelude::*;

mod m20240125_230737_create_users;
mod m20240125_232114_create_teams;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
  fn migrations() -> Vec<Box<dyn MigrationTrait>> {
    vec![
            Box::new(m20240125_230737_create_users::Migration),
            Box::new(m20240125_232114_create_teams::Migration),
        ]
  }
}
