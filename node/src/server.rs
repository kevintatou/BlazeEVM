use axum::{Router, serve};
use tokio::net::TcpListener;
use std::net::SocketAddr;

use crate::rpc;

/// Starts the JSON-RPC server on the specified address
pub async fn start(addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .merge(rpc::routes());

    let listener = TcpListener::bind(addr).await?;
    println!("Server listening on {}", addr);
    
    serve(listener, app).await?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{timeout, Duration};

    #[tokio::test]
    async fn test_server_starts() {
        let addr: SocketAddr = "127.0.0.1:0".parse().unwrap();
        let listener = TcpListener::bind(addr).await.unwrap();
        let bound_addr = listener.local_addr().unwrap();
        
        let server_handle = tokio::spawn(async move {
            let app = Router::new().merge(rpc::routes());
            serve(listener, app).await
        });

        // Give the server a moment to start
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Try to connect to the server
        let connect_result = timeout(
            Duration::from_secs(1),
            TcpListener::bind(bound_addr)
        ).await;

        // Should fail to bind because server is already listening
        assert!(connect_result.is_ok());
        assert!(connect_result.unwrap().is_err());

        // Clean up
        server_handle.abort();
    }
}
