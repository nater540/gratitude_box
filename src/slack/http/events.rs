use actix_web::{web, HttpResponse};
use sea_orm::*;

use crate::db::{DbPool, models::*};
use crate::slack::responses::*;
use crate::error::GBError;

pub async fn handler(pool: web::Data<DbPool>, message: web::Json<SlackResponse>) -> Result<HttpResponse, GBError> {
  let message = message.into_inner();
  let pool = pool.into_inner();

  let reaction_event = message.parse_event::<ReactionEvent>()?;

  let new_user = user::ActiveModel {
    slack_id: Set(reaction_event.user.clone()),
    slack_team_id: Set(message.team_id.clone()),
    ..Default::default()
  };

  let new_user = new_user.insert(pool.as_ref()).await?;

  Ok(HttpResponse::Ok().finish())
}
