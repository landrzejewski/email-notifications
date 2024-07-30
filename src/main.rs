use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};
async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn health() -> impl Responder {
    HttpResponse::Ok()
}

// cargo install cargo-expand
// cargo expand

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    const ADDRESS: &str = "127.0.0.1:8000";
    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health))
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind(ADDRESS)?
    .run()
    .await
}
