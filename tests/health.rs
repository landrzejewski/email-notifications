mod utils;

use crate::utils::spawn_app;

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
