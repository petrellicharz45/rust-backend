use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/stream")
            .route("/start", web::post().to(start_stream))
    );
}

async fn start_stream() -> impl actix_web::Responder {
    "Stream endpoint works!"
}