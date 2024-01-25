use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(Team::Table)
          .if_not_exists()
          .col(ColumnDef::new(Team::Id).uuid().not_null().primary_key())
          .col(ColumnDef::new(Team::SlackId).string().not_null())
          .col(ColumnDef::new(Team::ApiKey).string().not_null())
          .col(ColumnDef::new(Team::UpdatedAt).timestamp().not_null().default("now()"))
          .to_owned()
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(Team::Table).to_owned())
      .await
  }
}

#[derive(DeriveIden)]
enum Team {
  Table,
  Id,
  SlackId,
  ApiKey,
  UpdatedAt
}
