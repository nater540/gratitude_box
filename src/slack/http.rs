use actix_web::web;

mod events;

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/slack")
      .service(
        web::resource("events").route(web::post().to(events::handler))
      )
  );
}

pub mod headers {
  pub const SLACK_SIGNATURE_HEADER: &str = "x-slack-signature";
  pub const SLACK_REQUEST_TIMESTAMP_HEADER: &str = "x-slack-request-timestamp";
}
