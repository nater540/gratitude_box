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
