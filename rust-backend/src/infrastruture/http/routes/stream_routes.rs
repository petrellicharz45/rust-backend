use actix_web::{web, HttpResponse, Responder, Scope};

async fn stream_handler() -> impl Responder {
    HttpResponse::Ok().body("Stream response")
}

pub fn stream_routes() -> Scope {
    web::scope("/stream")
        .route("", web::get().to(stream_handler))
}