use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum SlackMessage {
  #[serde(rename = "url_verification")]
  UrlVerification { token: String, challenge: String },
  #[serde(rename = "event_callback")]
  EventCallback {
    token: String,
    team_id: String,
    event: serde_json::Value,
    event_id: String,
    event_time: i64
  }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReactionEvent {
  #[serde(rename = "type")]
  pub event_type: String,
  pub user: String,
  pub reaction: String,
  pub item_user: String,
  pub item: ReactionItem,
  pub event_ts: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReactionItem {
  #[serde(rename = "type")]
  pub item_type: String,
  pub channel: String,
  pub ts: String
}
