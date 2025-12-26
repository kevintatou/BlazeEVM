use axum::{extract::State, routing::post, Json, Router};
use blazeevm_core::chain::Chain;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Shared application state containing the chain
pub type AppState = Arc<RwLock<Chain>>;

#[derive(Serialize, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub method: String,
    pub params: Vec<serde_json::Value>,
    pub id: serde_json::Value,
}

/// Handler for eth RPC methods
async fn eth_handler(
    State(chain): State<AppState>,
    Json(request): Json<JsonRpcRequest>,
) -> Json<serde_json::Value> {
    match request.method.as_str() {
        "eth_blockNumber" => {
            let chain = chain.read().await;
            let block_number = chain.block_number();

            Json(serde_json::json!({
                "jsonrpc": "2.0",
                "result": format!("0x{:x}", block_number),
                "id": request.id
            }))
        }
        _ => Json(serde_json::json!({
            "jsonrpc": "2.0",
            "error": {
                "code": -32601,
                "message": format!("Method not found: {}", request.method)
            },
            "id": request.id
        })),
    }
}

/// Creates the eth RPC endpoint routes
pub fn routes(chain: AppState) -> Router {
    Router::new()
        .route("/", post(eth_handler))
        .with_state(chain)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_eth_block_number_genesis() {
        let chain = Arc::new(RwLock::new(Chain::new()));
        let app = routes(chain);

        let request_body = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "eth_blockNumber",
            "params": [],
            "id": 1
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_string(&request_body).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json_response: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json_response["jsonrpc"], "2.0");
        assert_eq!(json_response["result"], "0x0");
        assert_eq!(json_response["id"], 1);
    }

    #[tokio::test]
    async fn test_eth_block_number_with_blocks() {
        let mut chain = Chain::new();
        // Add a block with number 1
        chain.append_block(blazeevm_core::block::Block::with_header(
            1,
            primitive_types::H256::zero(),
            primitive_types::H256::zero(),
            1000,
        ));

        let chain = Arc::new(RwLock::new(chain));
        let app = routes(chain);

        let request_body = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "eth_blockNumber",
            "params": [],
            "id": 42
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_string(&request_body).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json_response: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json_response["jsonrpc"], "2.0");
        assert_eq!(json_response["result"], "0x1");
        assert_eq!(json_response["id"], 42);
    }

    #[tokio::test]
    async fn test_unsupported_method() {
        let chain = Arc::new(RwLock::new(Chain::new()));
        let app = routes(chain);

        let request_body = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "eth_unsupportedMethod",
            "params": [],
            "id": 1
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_string(&request_body).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json_response: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json_response["jsonrpc"], "2.0");
        assert!(json_response["error"].is_object());
        assert_eq!(json_response["error"]["code"], -32601);
        assert!(json_response["error"]["message"].as_str().unwrap().contains("Method not found"));
    }
}

