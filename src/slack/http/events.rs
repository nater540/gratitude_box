use actix_web::{web, HttpRequest, HttpResponse};
use sea_orm::*;

use crate::db::{DbPool, models::*};
use crate::slack::messages::*;
use crate::error::GBError;
use crate::http::AppState;

pub async fn handler(req: HttpRequest, state: web::Data<AppState>, message: web::Json<SlackMessage>) -> Result<HttpResponse, GBError> {
  let message = message.into_inner();
  // let pool = pool.into_inner();

  // TODO: Should be a better way to handle this, since I think slack sends
  // a challenge message for each configured event / command / etc..
  match message {
    SlackMessage::UrlVerification { challenge, .. } => {
      return Ok(HttpResponse::Ok().body(challenge));
    },
    _ => {
      dbg!(message);
    }
  }

  // let new_user = user::ActiveModel {
  //   slack_id: Set(reaction_event.user.clone()),
  //   slack_team_id: Set(message.team_id.clone()),
  //   ..Default::default()
  // };

  // let new_user = new_user.insert(pool.as_ref()).await?;

  Ok(HttpResponse::Ok().finish())
}
