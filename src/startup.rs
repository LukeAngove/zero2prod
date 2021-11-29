use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

use crate::routes::health_check;
use crate::routes::subscribe;

pub fn run(listener: TcpListener, db: PgPool) -> Result<Server, std::io::Error> {
    let db = web::Data::new(db);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/health-check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(db.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
