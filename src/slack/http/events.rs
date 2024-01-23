use actix_web::{web, Error, HttpResponse};
use crate::slack::responses::*;
use crate::error::GBError;

pub async fn handler(team_id: web::Path<String>, message: web::Json<SlackResponse>) -> Result<HttpResponse, GBError> {

  dbg!(team_id);
  let reaction_event = message.parse_event::<ReactionEvent>()?;

  dbg!(reaction_event);

  Ok(HttpResponse::Ok().finish())
}
