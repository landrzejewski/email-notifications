use std::io;
use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;

use crate::routes::{health, subscribe};

pub fn listen(listener: TcpListener, db_pool: PgPool) -> Result<Server, io::Error> {
    // Each worker runs its own copy of the application built by HttpServer calling the very same closure that
    // HttpServer::new takes as argument. That is why connection has to be cloneable
    // web::Data wraps our connection in an Atomic Reference Counted pointer
    let db_pool_data = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health", web::get().to(health))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(db_pool_data.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
