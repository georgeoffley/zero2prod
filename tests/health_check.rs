use actix_web::web::resource;
use zero2prod::run;

#[tokio::test]
async fn health_check_works() {
    spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get("http://localhost:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
    let server = run().expect("Failed to bind address");
    let _ = tokio::spawn(server);
}