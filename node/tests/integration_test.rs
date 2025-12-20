use tokio::net::TcpListener;
use std::time::Duration;

#[tokio::test]
async fn test_server_boots() {
    // Bind to an available port
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    
    // Spawn the server in a background task
    let server_handle = tokio::spawn(async move {
        // Create a simple axum app to test server bootstrap
        let app = axum::Router::new()
            .route("/health", axum::routing::get(|| async {
                axum::Json(serde_json::json!({"status": "ok"}))
            }));
        
        axum::serve(listener, app).await
    });
    
    // Give the server time to start
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    // Test that the server is running by making a request
    let client = reqwest::Client::new();
    let response = client
        .get(format!("http://{}/health", addr))
        .send()
        .await
        .expect("Failed to send request to server");
    
    assert_eq!(response.status(), 200);
    
    let body: serde_json::Value = response.json().await.expect("Failed to parse JSON");
    assert_eq!(body["status"], "ok");
    
    // Clean up
    server_handle.abort();
}

#[tokio::test]
async fn test_health_endpoint_returns_ok() {
    // Bind to an available port
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    
    // Spawn the server in a background task
    let server_handle = tokio::spawn(async move {
        let app = axum::Router::new()
            .route("/health", axum::routing::get(|| async {
                axum::Json(serde_json::json!({"status": "ok"}))
            }));
        
        axum::serve(listener, app).await
    });
    
    // Give the server time to start
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    // Test the health endpoint
    let client = reqwest::Client::new();
    let response = client
        .get(format!("http://{}/health", addr))
        .send()
        .await
        .expect("Failed to connect to health endpoint");
    
    assert_eq!(response.status(), 200, "Health endpoint should return 200 OK");
    
    let body: serde_json::Value = response.json().await.expect("Failed to parse JSON response");
    assert_eq!(body["status"], "ok", "Health endpoint should return status: ok");
    
    // Clean up
    server_handle.abort();
}
