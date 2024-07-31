use email_notifications::configuration::get_config;
use email_notifications::startup::listen;
use sqlx::PgPool;
use std::net::TcpListener;

// cargo expand --test health

#[tokio::test]
async fn health_returns_status_ok() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/health", &test_app.address))
        .send()
        .await
        .expect("Failed to execute request");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let test_app = spawn_app().await;
    sqlx::query!("DELETE FROM subscriptions",)
        .execute(&test_app.db_pool)
        .await
        .expect("Failed to delete saved subscriptions");
    let client = reqwest::Client::new();
    let body = "name=jan&email=jan.kowalski%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &test_app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");
    assert_eq!(200, response.status().as_u16());
    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&test_app.db_pool)
        .await
        .expect("Failed to fetch saved subscription");
    assert_eq!(saved.email, "jan.kowalski@gmail.com");
    assert_eq!(saved.name, "jan");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let test_app = spawn_app().await.address;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=jan", "missing the name"),
        ("email=jan.kowalski%40gmail.com", "missing the email"),
        ("", "missing both name and email"),
    ];
    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &test_app))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}",
            error_message
        );
    }
}

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
