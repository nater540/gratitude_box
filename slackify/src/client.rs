use serde::{de::DeserializeOwned, Serialize};
use std::borrow::Cow;

pub mod chat;

use crate::error::{SlackifyError, Result};

#[derive(Debug)]
pub struct ClientBuilder<'a> {
  token: Option<Cow<'a, str>>,
  base_url: Option<Cow<'a, str>>
}

impl<'a> Default for ClientBuilder<'a> {
  fn default() -> ClientBuilder<'a> {
    Self { token: None, base_url: Some(Cow::Borrowed("https://slack.com/api")) }
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

  #[inline]
  pub fn base_url<S>(&mut self, base_url: S) -> &mut Self
    where S: Into<Cow<'a, str>> {
    self.base_url = Some(base_url.into());
    self
  }

  /// Consumes the builder & creates a new client.
  pub fn create(&self) -> Result<Client> {
    use reqwest::header::{HeaderValue, HeaderMap, AUTHORIZATION};

    let token = match self.token {
      Some(ref tok) => tok.to_owned().to_string(),
      None          => return Err(SlackifyError::BuilderError("Must specify `token`".to_string()))
    };

    let base_url = match self.base_url {
      Some(ref url) => url.to_owned().to_string(),
      None          => return Err(SlackifyError::BuilderError("Must specify `base_url`".to_string()))
    };

    // Add the AUTHORIZATION header and mark it as sensitive so it doesn't get logged
    let mut auth_header = HeaderValue::from_str(&format!("Bearer {}", token))?;
    auth_header.set_sensitive(true);

    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, auth_header);

    let client = reqwest::Client::builder()
      .default_headers(headers)
      .build()?;

    Ok(Client { base_url, client })
  }
}

#[derive(Debug)]
pub struct Client {
  base_url: String,
  client: reqwest::Client
}

impl Client {
  pub(crate) async fn post<B, R>(&self, url: &str, body: B) -> Result<R>
  where B: Serialize + Send + Sync,
        R: DeserializeOwned + Send {
    let response = self
      .client
      .post(url)
      .json(&body)
      .send()
      .await?;

    if !response.status().is_success() {
      return Err(SlackifyError::RequestFailed(
        reqwest::StatusCode::from(response.status())
      ));
    }

    Ok(response.json::<R>().await?)
  }
}
