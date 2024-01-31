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

// pub struct SlackMessage {
//   pub token: String,
//   pub team_id: String,
//   pub event: serde_json::Value,

//   #[serde(rename = "type")]
//   pub event_type: String,
//   pub event_id: String,
//   pub event_time: i64
// }

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

// impl SlackMessage {
//   pub fn parse_event<T: for<'de> Deserialize<'de>>(&self) -> anyhow::Result<T> {
//     serde_json::from_value(self.event.clone()).map_err(Into::into)
//   }
// }