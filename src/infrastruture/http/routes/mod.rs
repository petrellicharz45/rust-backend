use actix_web::web;

pub mod broadcast_routes;


pub mod stream_routes;


pub fn init_routes(cfg: &mut web::ServiceConfig) {
    // Add a health check endpoint
    cfg.service(
        web::resource("/health")
            .route(web::get().to(|| async { "Server is running!" }))
    );
    
    // Configure your other routes
    broadcast_routes::configure(cfg);
    stream_routes::configure(cfg);
}