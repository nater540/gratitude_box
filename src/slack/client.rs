use serde::{de::DeserializeOwned, Serialize};
use std::borrow::Cow;

#[derive(thiserror::Error, Debug)]
pub enum ClientError {
  #[error("failed to build client ({0})")]
  BuilderError(String),

  #[error("failed to send request")]
  ReqwestError(#[from] reqwest::Error),

  #[error("invalid request header")]
  InvalidRequestHeader(#[from] reqwest::header::InvalidHeaderValue),

  #[error("failed to deserialize response")]
  DeserializeError(#[from] serde_json::Error)
}

pub type Result<T, E = ClientError> = std::result::Result<T, E>;

#[derive(Debug)]
pub struct ClientBuilder<'a> {
  token: Option<Cow<'a, str>>
}

impl<'a> Default for ClientBuilder<'a> {
  fn default() -> ClientBuilder<'a> {
    Self { token: None }
  }
}

impl<'a> ClientBuilder<'a> {
  pub fn from_token<S>(token: S) -> Result<Client>
    where S: Into<Cow<'a, str>> {
    ClientBuilder::default().token(token).create()
  }

  #[inline]
  pub fn token<S>(&mut self, token: S) -> &mut Self
    where S: Into<Cow<'a, str>> {
    self.token = Some(token.into());
    self
  }

  /// Consumes the builder & creates a new client.
  pub fn create(&self) -> Result<Client> {
    use reqwest::header::{HeaderValue, HeaderMap, AUTHORIZATION};

    let token = match self.token {
      Some(ref tok) => tok.to_owned().to_string(),
      None          => return Err(ClientError::BuilderError("Must specify `token`".to_string()))
    };

    // Add the AUTHORIZATION header and mark it as sensitive so it doesn't get logged
    let mut auth_header = HeaderValue::from_str(&format!("Bearer {}", token))?;
    auth_header.set_sensitive(true);

    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, auth_header);

    let client = reqwest::Client::builder()
      .default_headers(headers)
      .build()?;

    Ok(Client { token, client })
  }
}

#[derive(Debug)]
pub struct Client {
  token: String, // TODO: I don't _think_ we need this, but I'm keeping it around in case we need to reestablish the connection
  client: reqwest::Client
}

impl Client {
  async fn post<P: Serialize>(&self, url: &str, params: P) -> Result<reqwest::Response> {
    Ok(
      self
        .client
        .post(url)
        .json(&params)
        .send()
        .await?
    )
  }
}
