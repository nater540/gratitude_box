use sea_orm::{ActiveValue, Set, entity::prelude::*};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  pub id: Uuid,
  pub slack_id: String,
  pub slack_team_id: String,
  pub updated_at: DateTime,
  pub points: i32
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
