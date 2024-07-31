use std::io;
use std::net::TcpListener;

use sqlx::PgPool;

use email_notifications::configuration::get_config;
use email_notifications::startup::listen;

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let config = get_config().expect("Failed to read configuration file");
    let db_pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");
    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address)?;
    listen(listener, db_pool)?.await
}
