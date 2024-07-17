#[tokio::test]
async fn test_health_check() {
    // Arrange
    let host = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("http://{host}/health_check"))
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let host = spawn_app().await;
    let client = reqwest::Client::new();
    let body = "name=marcin&email=mail%40marszy.com";
    // let db_connection = 

    // Act
    let response = client
        .post(&format!("http://{host}/subscriptions"))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");
    
    // Arrange
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_a_422_when_data_is_missing() {
    // Arrange
    let host = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=marcin", "missing mail"),
        ("email=mail%40marszy.com", "missing name"),
        ("", "missing both name and email")
    ];

    // Act
    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("http://{host}/subscriptions"))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");
        
        // Arrange
        assert_eq!(422, response.status().as_u16(), 
            "The API did not fail with 400 Bad Request when the payload was {}",
        error_message);
    }
}

async fn spawn_app() -> String {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    tokio::spawn(zero_to_prod::run(listener));

    format!("127.0.0.1:{port}")
}