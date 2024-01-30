use actix_web::{error::ResponseError, HttpResponse, http::StatusCode};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GBError {
  #[error("Internal error")]
  InternalError(#[from] anyhow::Error),

  #[error("Database error")]
  DatabaseError(#[from] sea_orm::DbErr),

  #[error("Missing Slack header")]
  SlackSignatureFailure
}

impl ResponseError for GBError {
  fn error_response(&self) -> HttpResponse {
    let (status_code, details) = match *self {
      GBError::InternalError(ref err) => {
        (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
      },
      GBError::DatabaseError(ref err) => {
        (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
      },
      GBError::SlackSignatureFailure => {
        (StatusCode::BAD_REQUEST, "Failed to verify request signature".to_string())
      }
    };

    let error_message = serde_json::json!({
      "error": self.to_string(),
      "details": details
    });

    HttpResponse::build(status_code).json(error_message)
  }
}
