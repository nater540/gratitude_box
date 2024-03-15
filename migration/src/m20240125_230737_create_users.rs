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
          .table(Users::Table)
          .if_not_exists()
          .col(ColumnDef::new(Users::Id).uuid().not_null().primary_key())
          .col(ColumnDef::new(Users::TeamId).uuid().not_null())
          .col(ColumnDef::new(Users::SlackId).string().not_null())
          .col(ColumnDef::new(Users::Points).integer().not_null().default("0"))
          .col(ColumnDef::new(Users::IsAdmin).boolean().not_null().default(false))
          .col(ColumnDef::new(Users::UpdatedAt).timestamp().not_null().default("now()"))
          .to_owned()
      )
      .await?;

      manager.create_index(sea_query::Index::create()
        .if_not_exists()
        .table(Users::Table)
        .name("idx_user_team_id")
        .col(Users::TeamId)
        .to_owned()
      ).await?;

      manager.create_index(sea_query::Index::create()
        .if_not_exists()
        .table(Users::Table)
        .name("idx_user_slack_id")
        .col(Users::SlackId)
        .to_owned()
      ).await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(Users::Table).to_owned())
      .await
  }
}

#[derive(DeriveIden)]
enum Users {
  Table,
  Id,
  TeamId,
  SlackId,
  Points,
  IsAdmin,
  UpdatedAt
}
