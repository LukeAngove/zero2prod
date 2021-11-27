mod health_check;
mod subscriptions;

pub use health_check::*;
pub use subscriptions::*;

use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health-check", web::get().to(health_check))
            .route("/subscriptions", web::get().to(subscribe))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
