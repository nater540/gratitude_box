use actix_web::{web, HttpRequest, HttpResponse};
use sea_orm::*;

use crate::db::{DbPool, entities::*};
use crate::slack::messages::*;
use crate::error::GBError;
use crate::http::AppState;

pub async fn handler(req: HttpRequest, state: web::Data<AppState>, message: web::Json<SlackMessage>) -> Result<HttpResponse, GBError> {
  let message = message.into_inner();

  // If this is a challenge, respond accordingly
  match message {
    SlackMessage::UrlVerification { challenge, .. } => {
      return Ok(HttpResponse::Ok().body(challenge));
    },
    SlackMessage::EventCallback(message) => {
      let team = Teams::find_by_slack_id(&state.db, &message.team_id).await?;

      if let Some(team) = team {
        match message.event {
          EventWrapper::ReactionAdded(event) => {
            return handle_reaction_added(&state, team, event).await;    // TODO: Combine?
          },
          EventWrapper::ReactionRemoved(event) => {
            return handle_reaction_removed(&state, team, event).await;  // TODO: Combine?
          },
          EventWrapper::Message(event) => {
            return handle_message(&state, team, event).await;
          }
        }
      }
      else {
        tracing::warn!("Received event for unknown team: `{}`", message.team_id);
        return Ok(HttpResponse::Ok().finish());
      }

    }
  }
}

async fn handle_reaction_added(state: &AppState, team: Team, event: ReactionEvent) -> Result<HttpResponse, GBError> {
  tracing::info!("Reaction added: {:?}", &event);

  let mut user = Users::find_or_create(&state.db, &event.item_user, team.id).await?;
  tracing::info!("User: {:?}", &user);

  Ok(HttpResponse::Ok().finish())
}

async fn handle_reaction_removed(state: &AppState, team: Team, event: ReactionEvent) -> Result<HttpResponse, GBError> {
  tracing::info!("Reaction removed: {:?}", event);
  Ok(HttpResponse::Ok().finish())
}

async fn handle_message(state: &AppState, team: Team, event: MessageEvent) -> Result<HttpResponse, GBError> {
  tracing::info!("Message: {:?}", event);
  Ok(HttpResponse::Ok().finish())
}
