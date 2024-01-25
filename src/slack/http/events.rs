use actix_web::{web, HttpResponse};
use crate::slack::responses::*;
use crate::error::GBError;
use crate::db::{DbPool, models::*};

pub async fn handler(_pool: web::Data<DbPool>, message: web::Json<SlackResponse>) -> Result<HttpResponse, GBError> {
  let message = message.into_inner();

  let reaction_event = message.parse_event::<ReactionEvent>()?;
  dbg!(reaction_event);

  // let mut conn = pool.get().await?;
  // let user = web::block(move || {
  //   let pg_conn = conn.get_mut();
  //   user::find_or_create(pg_conn, &reaction_event.user, &message.team_id).unwrap()
  // })
  // .await?;

  Ok(HttpResponse::Ok().finish())
}
