use axum::{serve, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;

use crate::rpc;

/// Starts the JSON-RPC server on the specified address
pub async fn start(addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
    let app = create_app();
    let listener = TcpListener::bind(addr).await?;
    println!("Server listening on {}", addr);

    serve(listener, app).await?;

    Ok(())
}

/// Creates the application router with all routes
pub fn create_app() -> Router {
    Router::new().merge(rpc::routes())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{timeout, Duration};

    #[tokio::test]
    async fn test_server_starts() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        let server_handle = tokio::spawn(async move {
            let app = create_app();
            serve(listener, app).await
        });

        // Give the server a moment to start
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Verify the server is running by connecting to it
        let connect_result =
            timeout(Duration::from_secs(1), tokio::net::TcpStream::connect(addr)).await;

        // Should successfully connect to the server
        assert!(connect_result.is_ok(), "Server should accept connections");
        assert!(
            connect_result.unwrap().is_ok(),
            "Connection to server should succeed"
        );

        // Clean up
        server_handle.abort();
    }
}
