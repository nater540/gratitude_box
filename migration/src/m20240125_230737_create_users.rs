use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    let db = manager.get_connection();
    db.execute_unprepared("CREATE EXTENSION IF NOT EXISTS \"uuid-ossp\";").await?;

    manager
      .create_table(
        Table::create()
          .table(User::Table)
          .if_not_exists()
          .col(ColumnDef::new(User::Id).uuid().not_null().primary_key())
          .col(ColumnDef::new(User::SlackId).string().not_null())
          .col(ColumnDef::new(User::SlackTeamId).string().not_null())
          .col(ColumnDef::new(User::UpdatedAt).timestamp().not_null().default("now()"))
          .col(ColumnDef::new(User::Points).integer().not_null().default("0"))
          .to_owned()
      )
      .await?;

      manager.create_index(sea_query::Index::create()
        .if_not_exists()
        .table(User::Table)
        .name("idx_user_slack_id")
        .col(User::SlackId)
        .col(User::SlackTeamId)
        .to_owned()
      ).await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(User::Table).to_owned())
      .await
  }
}

#[derive(DeriveIden)]
enum User {
  Table,
  Id,
  SlackId,
  SlackTeamId,
  UpdatedAt,
  Points
}
