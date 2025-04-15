use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/broadcast")
            .route("/send", web::post().to(send_broadcast))
    );
}

async fn send_broadcast() -> impl actix_web::Responder {
    "Broadcast endpoint works!"
}