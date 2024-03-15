use sea_orm::{ActiveValue, Set, entity::prelude::*};
use anyhow::Result;
use uuid::Uuid;

use crate::db::DbPool;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "users")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  pub id: Uuid,
  pub team_id: Uuid,
  pub slack_id: String,
  pub points: i32,
  pub is_admin: bool,
  pub updated_at: DateTime
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

// https://www.sea-ql.org/SeaORM/docs/generate-entity/entity-structure
impl ActiveModelBehavior for ActiveModel {
  fn new() -> Self {
    Self {
      id: Set(Uuid::new_v4()),
      points: ActiveValue::set(0),
      ..ActiveModelTrait::default()
    }
  }
}

impl Entity {
  /// Find or create a new user record.
  ///
  /// # Parameters
  /// - `conn`: Reference to the PG connection.
  /// - `slack_id`: The Slack ID for the user.
  /// - `team_id`: The team ID for the user.
  ///
  /// # Returns
  /// This returns a `Result<Model>` that contains either an existing user record,
  /// or a user record that was just created based on the provided params.
  ///
  /// # Errors
  /// This function should only error in extreme cases due to PG constraint failures, network errors, etc.
  pub async fn find_or_create<T>(conn: &DbPool, slack_id: T, team_id: Uuid) -> Result<Model>
    where T: Into<String> {

    let slack_id = slack_id.into();

    // Try to find an existing user
    let user = Entity::find()
      .filter(Column::SlackId.eq(&*slack_id))
      .filter(Column::TeamId.eq(team_id))
      .one(conn)
      .await?;

    match user {
      Some(usr) => Ok(usr),
      None => {
        let new_user = ActiveModel {
          slack_id: Set(slack_id),
          team_id: Set(team_id),
          ..Default::default()
        };

        Ok(new_user.insert(conn).await?)
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const TEST_SLACK_ID: &str = "U123ABC456";
  const TEST_SLACK_TEAM_ID: &str = "TAABBCCDDEE";

}
