use actix_web::{web, HttpResponse, Responder};

pub fn broadcast_route(cfg: &mut web::ServiceConfig) {
    cfg.route("/broadcast", web::get().to(broadcast_handler));
}

async fn broadcast_handler() -> impl Responder {
    HttpResponse::Ok().body("Broadcast route reached!")
}