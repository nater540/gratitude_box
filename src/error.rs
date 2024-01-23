use actix_web::{error::ResponseError, HttpResponse, http::StatusCode};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GBError {
  #[error("Internal error")]
  InternalError(#[from] anyhow::Error)
}

impl ResponseError for GBError {
  fn error_response(&self) -> HttpResponse {
    match *self {
      GBError::InternalError(ref _e) => {
        HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
      }
    }
  }
}
