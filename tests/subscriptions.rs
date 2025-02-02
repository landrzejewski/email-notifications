mod utils;

use crate::utils::spawn_app;

// cargo expand --test health

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
