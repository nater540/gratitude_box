#[derive(thiserror::Error, Debug)]
pub enum SlackifyError {
  #[error("failed to build ({0})")]
  BuilderError(String),

  #[error("failed to send request")]
  ReqwestError(#[from] reqwest::Error),

  #[error("request failed with status code {0}")]
  RequestFailed(reqwest::StatusCode),

  #[error("invalid request header")]
  InvalidRequestHeader(#[from] reqwest::header::InvalidHeaderValue),

  #[error("failed to deserialize response")]
  DeserializeError(#[from] serde_json::Error)
}

pub type Result<T, E = SlackifyError> = std::result::Result<T, E>;
