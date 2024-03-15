use sea_orm::{ActiveValue, Set, entity::prelude::*};
use anyhow::Result;
use uuid::Uuid;

use crate::db::DbPool;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "teams")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  pub id: Uuid,
  pub slack_id: String,
  pub api_token: String,
  pub updated_at: DateTime
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {
  fn new() -> Self {
    Self {
      id: Set(Uuid::new_v4()),
      ..ActiveModelTrait::default()
    }
  }
}

impl Entity {
  /// Finds a team record by its Slack ID.
  ///
  /// # Parameters
  /// - `conn`: Reference to the PG connection.
  /// - `slack_id`: The Slack ID for the team.
  ///
  /// # Returns
  /// This returns a `Result<Option<Model>>` that contains the team record if found.
  pub async fn find_by_slack_id<T>(conn: &DbPool, slack_id: T) -> Result<Option<Model>>
    where T: Into<String> {

    Ok(Entity::find()
      .filter(Column::SlackId.eq(&*slack_id.into()))
      .one(conn)
      .await?)
  }
}
