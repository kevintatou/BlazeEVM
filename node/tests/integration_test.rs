use blazeevm_node::server;
use std::time::Duration;
use tokio::net::TcpListener;

/// Helper function to spawn a test server using production code and return its address and handle
async fn spawn_test_server() -> (
    std::net::SocketAddr,
    tokio::task::JoinHandle<Result<(), std::io::Error>>,
) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    let server_handle = tokio::spawn(async move {
        let app = server::create_app();
        axum::serve(listener, app).await
    });

    // Give the server time to start
    tokio::time::sleep(Duration::from_millis(100)).await;

    (addr, server_handle)
}

#[tokio::test]
async fn test_server_boots() {
    let (addr, server_handle) = spawn_test_server().await;

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
    let (addr, server_handle) = spawn_test_server().await;

    // Test the health endpoint
    let client = reqwest::Client::new();
    let response = client
        .get(format!("http://{}/health", addr))
        .send()
        .await
        .expect("Failed to connect to health endpoint");

    assert_eq!(
        response.status(),
        200,
        "Health endpoint should return 200 OK"
    );

    let body: serde_json::Value = response
        .json()
        .await
        .expect("Failed to parse JSON response");
    assert_eq!(
        body["status"], "ok",
        "Health endpoint should return status: ok"
    );

    // Clean up
    server_handle.abort();
}

#[tokio::test]
async fn test_eth_chain_id_endpoint() {
    let (addr, server_handle) = spawn_test_server().await;

    // Test the eth_chainId JSON-RPC endpoint
    let client = reqwest::Client::new();

    let request_body = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "eth_chainId",
        "params": [],
        "id": 1
    });

    let response = client
        .post(format!("http://{}/", addr))
        .json(&request_body)
        .send()
        .await
        .expect("Failed to send eth_chainId request");

    assert_eq!(
        response.status(),
        200,
        "eth_chainId endpoint should return 200 OK"
    );

    let body: serde_json::Value = response
        .json()
        .await
        .expect("Failed to parse JSON response");

    assert_eq!(body["jsonrpc"], "2.0", "JSON-RPC version should be 2.0");
    assert_eq!(
        body["result"], "0x539",
        "Chain ID should be 0x539 (1337 in hex)"
    );
    assert_eq!(body["id"], 1, "Response ID should match request ID");

    // Clean up
    server_handle.abort();
}

#[tokio::test]
async fn test_eth_block_number_endpoint() {
    let (addr, server_handle) = spawn_test_server().await;

    // Test the eth_blockNumber JSON-RPC endpoint
    let client = reqwest::Client::new();

    let request_body = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "eth_blockNumber",
        "params": [],
        "id": 1
    });

    let response = client
        .post(format!("http://{}/", addr))
        .json(&request_body)
        .send()
        .await
        .expect("Failed to send eth_blockNumber request");

    assert_eq!(
        response.status(),
        200,
        "eth_blockNumber endpoint should return 200 OK"
    );

    let body: serde_json::Value = response
        .json()
        .await
        .expect("Failed to parse JSON response");

    assert_eq!(body["jsonrpc"], "2.0", "JSON-RPC version should be 2.0");
    assert_eq!(
        body["result"], "0x0",
        "Block number should be 0x0 (genesis block)"
    );
    assert_eq!(body["id"], 1, "Response ID should match request ID");

    // Clean up
    server_handle.abort();
}
