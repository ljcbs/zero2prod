use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request");

    // We check that the response has the 200 OK status code
    assert!(response.status().is_success());
    // We check that the response has an empty body
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    // Randomize port to avoid conflicts in parallel test runs
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address");
    // Spawn our application as a background task
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
