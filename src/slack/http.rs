use axum::{Router, routing::post};

pub fn router() -> Router {
  async fn interaction() -> &'static str {
    "This is the song that never ends, it goes on and on my friends..."
  }

  Router::new()
    .route("/slack/interaction", post(interaction))
}
