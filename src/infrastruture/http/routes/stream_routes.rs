use actix_web::{web, HttpResponse};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/stream")
            .route("/start", web::post().to(start_stream))
            .route("/stop", web::post().to(stop_stream))
            .route("/info", web::get().to(get_stream_info))
    );
}

async fn start_stream() -> HttpResponse {
    HttpResponse::Ok().json("Stream started successfully")
}

async fn stop_stream() -> HttpResponse {
    HttpResponse::Ok().json("Stream stopped successfully")
}

async fn get_stream_info() -> HttpResponse {
    HttpResponse::Ok().json("Stream information")
}