mod config;
mod infrastruture;
use infrastruture::http::routes::init_routes;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Server starting at http://127.0.0.1:8000");

    HttpServer::new(|| {
        App::new()
            .configure(init_routes)
            // Add a default service for unhandled routes
            .default_service(
                web::route().to(|| async { 
                    actix_web::HttpResponse::NotFound().body("Route not found") 
                })
            )
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}