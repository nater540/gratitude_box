use serde::{Serialize, Deserialize};
use derive_builder::Builder;

use super::{Client, Result};

impl Client {
  /// Sends a message to a channel.
  /// Example:
  /// ```rust
  /// let content = Message {
  ///   text: Some("This is the song that never ends, it goes on and on my friends....")
  /// };

  /// let message = ChatMessageBuilder::default()
  ///   .channel("#nates-science-lab")
  ///   .content(content)
  ///   .build()?;
  /// let response = message.send(&client).await?;
  /// ```

  pub async fn chat_post_message<'a>(&self, request: ChatMessage<'a>) -> Result<ChatMessageResponse> {
    let url = format!("{}/chat.postMessage", self.base_url);
    Ok(self.post(&url, request).await?)
  }
}

#[derive(Debug, Serialize, Builder)]
pub struct ChatMessage<'a> {
  #[builder(setter(into))]
  pub channel: &'a str,

  #[serde(flatten)]
  pub content: Message<'a>,

  // Emoji to use as the icon for this message. Overrides icon_url.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(setter(into, strip_option), default)]
  pub icon_emoji: Option<&'a str>,

  // URL to an image to use as the icon for this message.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(setter(into, strip_option), default)]
  pub icon_url: Option<&'a str>,

  // JSON object with event_type and event_payload fields, presented as a URL-encoded string. Metadata you post to Slack is accessible to any app or user who is a member of that workspace.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(setter(into, strip_option), default)]
  pub metadata: Option<serde_json::Value>,

  // Provide another message's ts value to make this message a reply. Avoid using a reply's ts value; use its parent instead.
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(setter(into, strip_option), default)]
  pub thread_ts: Option<&'a str>
}

impl<'a> ChatMessage<'a> {
  pub async fn send(self, client: &Client) -> Result<ChatMessageResponse> {
    client.chat_post_message(self).await
  }
}

#[derive(Debug, Deserialize)]
pub struct ChatMessageResponse {
  pub ok: bool,
  pub channel: Option<String>,
  pub error: Option<String>,
  pub ts: Option<String>
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct Message<'a> {
  pub text: Option<&'a str>
  // TODO: Support blocks => https://api.slack.com/block-kit
}
