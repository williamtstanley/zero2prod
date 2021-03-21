use std::net::TcpListener;

#[actix_rt::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();

    // We need reqwest for HTTP requests against our application
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health-check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success()); // 200 OK
    assert_eq!(Some(0), response.content_length()); // no body
}

fn spawn_app() -> String {
    // Port 0 will have the OS assign a random available portnumber
    // this will ensure that multiple parallel tests do not attempt to bind to the same port
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port.");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address");

    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
