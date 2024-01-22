use axum::{
  response::{IntoResponse, Response},
  http::StatusCode,
  Json
};
use anyhow::Error as AnyError;

#[derive(serde::Serialize)]
struct ErrorResponse {
  error: String
}

#[derive(Debug)]
pub struct AppError(AnyError);
pub type Result<T> = anyhow::Result<T, AppError>;

impl From<AnyError> for AppError {
  fn from(err: AnyError) -> Self {
    AppError(err)
  }
}

impl From<std::io::Error> for AppError {
  fn from(err: std::io::Error) -> Self {
    AppError(AnyError::new(err))
  }
}

impl From<serde_json::Error> for AppError {
  fn from(err: serde_json::Error) -> Self {
    AppError(AnyError::new(err))
  }
}

impl IntoResponse for AppError {
  fn into_response(self) -> Response {
    let response = ErrorResponse { error: self.0.to_string() };
    let code = StatusCode::INTERNAL_SERVER_ERROR;
    (code, Json(response)).into_response()
  }
}
