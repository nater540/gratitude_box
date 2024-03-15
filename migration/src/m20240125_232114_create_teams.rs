use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    let db = manager.get_connection();
    db.execute_unprepared("CREATE EXTENSION IF NOT EXISTS \"pgcrypto\";").await?;

    manager
      .create_table(
        Table::create()
          .table(Teams::Table)
          .if_not_exists()
          .col(ColumnDef::new(Teams::Id).uuid().not_null().primary_key())
          .col(ColumnDef::new(Teams::SlackId).string().not_null())
          .col(ColumnDef::new(Teams::ApiToken).string().not_null())
          .col(ColumnDef::new(Teams::TeamName).string())
          .col(ColumnDef::new(Teams::UpdatedAt).timestamp().not_null().default("now()"))
          .to_owned()
      )
      .await?;

    manager.create_index(sea_query::Index::create()
      .if_not_exists()
      .table(Teams::Table)
      .name("idx_team_slack_id")
      .col(Teams::SlackId)
      .to_owned()
    ).await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(Teams::Table).to_owned())
      .await
  }
}

#[derive(DeriveIden)]
enum Teams {
  Table,
  Id,
  SlackId,
  ApiToken,
  TeamName,
  UpdatedAt
}
