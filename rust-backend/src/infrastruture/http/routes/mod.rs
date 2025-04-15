use actix_web::web;

pub mod broadcast_route;
pub mod stream_route;

pub use broadcast_route::*;
pub use stream_route::*;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    broadcast_route::configure(cfg);
    stream_route::configure(cfg);
}