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
          .col(ColumnDef::new(Team::SigningSecret).string().not_null())
          .col(ColumnDef::new(Team::UpdatedAt).timestamp().not_null().default("now()"))
          .to_owned()
      )
      .await?;

    manager.create_index(sea_query::Index::create()
      .if_not_exists()
      .table(Team::Table)
      .name("idx_team_slack_id_api_key")
      .col(Team::SlackId)
      .col(Team::ApiKey)
      .to_owned()
    ).await
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
  SigningSecret,
  UpdatedAt
}
