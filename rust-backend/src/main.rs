mod config;
use actix_web::{App, HttpServer};
use infrastruture::http::routes::init_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Starting the server...");

    HttpServer::new(|| {
        App::new()
            .configure(init_routes)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
