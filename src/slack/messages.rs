use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum SlackMessage {
  #[serde(rename = "url_verification")]
  UrlVerification { token: String, challenge: String },

  #[serde(rename = "event_callback")]
  EventCallback(EventCallback)
}

#[derive(Debug, Deserialize)]
pub struct EventCallback {
  pub token: String,
  pub team_id: String,
  pub api_app_id: String,
  pub event: EventWrapper,
  pub event_id: String,
  pub event_time: u64
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum EventWrapper {
  #[serde(rename = "reaction_added")]
  ReactionAdded(ReactionEvent),

  #[serde(rename = "reaction_removed")]
  ReactionRemoved(ReactionEvent),

  #[serde(rename = "message")]
  Message(MessageEvent)
}

#[derive(Debug, Deserialize)]
pub struct ReactionEvent {
  pub user: String,
  pub reaction: String,
  pub item_user: String,
  pub item: Item,
  pub event_ts: String
}

#[derive(Debug, Deserialize)]
pub struct Item {
  #[serde(rename = "type")]
  pub item_type: String,
  pub channel: String,
  pub ts: String
}

#[derive(Debug, Deserialize)]
pub struct MessageEvent {
  pub subtype: Option<String>,
  pub text: String,
  pub ts: String,
  pub user: String
}
