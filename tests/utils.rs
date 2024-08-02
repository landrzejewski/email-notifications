use std::net::TcpListener;
use sqlx::PgPool;
use email_notifications::configuration::get_config;
use email_notifications::startup::listen;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

pub async fn spawn_app() -> TestApp {
    // Start at random port: host:0
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let config = get_config().expect("Failed to read configuration");
    let db_pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");
    let server = listen(listener, db_pool.clone()).expect("Failed to bind address");
    // Launch the server as a background task, tokio::spawn returns a handle to the spawned future
    let _ = tokio::spawn(server);
    TestApp {
        address: format!("http://127.0.0.1:{}", port),
        db_pool,
    }
}
