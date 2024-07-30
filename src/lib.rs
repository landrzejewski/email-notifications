use std::io;
use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};

async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn listen(listener: TcpListener) -> Result<Server, io::Error> {
    let server = HttpServer::new(|| App::new().route("/health", web::get().to(health)))
        .listen(listener)?
        .run();
    Ok(server)
}
