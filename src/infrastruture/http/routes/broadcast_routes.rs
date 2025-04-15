use actix_web::{web, HttpResponse};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/broadcast")
            .route("/send", web::post().to(handle_broadcast))
            .route("/status", web::get().to(get_broadcast_status))
    );
}

async fn handle_broadcast() -> HttpResponse {
    HttpResponse::Ok().json("Broadcast message received")
}

async fn get_broadcast_status() -> HttpResponse {
    HttpResponse::Ok().json("Broadcast system operational")
}