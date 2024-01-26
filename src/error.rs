use actix_web::{error::ResponseError, HttpResponse, http::StatusCode};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GBError {
  #[error("Internal error")]
  InternalError(#[from] anyhow::Error),

  #[error("Database error")]
  DatabaseError(#[from] sea_orm::DbErr)
}

impl ResponseError for GBError {
  fn error_response(&self) -> HttpResponse {
    let status_code = match *self {
      GBError::InternalError(ref _e) => StatusCode::INTERNAL_SERVER_ERROR,
      _ => StatusCode::INTERNAL_SERVER_ERROR
    };

    let error_message = serde_json::json!({
      "error": self.to_string(),
      "status_code": status_code.as_u16(),
    });

    HttpResponse::build(status_code).json(error_message)
  }
}
