use axum::{extract::State, routing::post, Json, Router};
use blazeevm_core::chain::Chain;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Configuration for the Ethereum RPC endpoints
#[derive(Clone, Debug)]
pub struct EthConfig {
    /// Chain ID (default: 1337 for local development)
    pub chain_id: u64,
    /// The blockchain instance
    pub chain: Chain,
}

impl Default for EthConfig {
    fn default() -> Self {
        Self {
            chain_id: 1337,
            chain: Chain::default(),
        }
    }
}

/// JSON-RPC 2.0 request structure
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub method: String,
    pub params: Option<serde_json::Value>,
    pub id: serde_json::Value,
}

/// JSON-RPC 2.0 response structure
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub result: serde_json::Value,
    pub id: serde_json::Value,
}

/// JSON-RPC 2.0 error response structure
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcError {
    pub jsonrpc: String,
    pub error: ErrorDetail,
    pub id: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorDetail {
    pub code: i32,
    pub message: String,
}

/// Handler for the JSON-RPC endpoint
async fn json_rpc_handler(
    State(config): State<Arc<EthConfig>>,
    Json(request): Json<JsonRpcRequest>,
) -> Json<serde_json::Value> {
    match request.method.as_str() {
        "eth_chainId" => {
            let response = JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                result: serde_json::json!(format!("0x{:x}", config.chain_id)),
                id: request.id.clone(),
            };
            match serde_json::to_value(response) {
                Ok(value) => Json(value),
                Err(_) => create_internal_error(request.id),
            }
        }
        "eth_blockNumber" => {
            let block_number = config.chain.get_block_number();
            let response = JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                result: serde_json::json!(format!("0x{:x}", block_number)),
                id: request.id.clone(),
            };
            match serde_json::to_value(response) {
                Ok(value) => Json(value),
                Err(_) => create_internal_error(request.id),
            }
        }
        _ => {
            let error_response = JsonRpcError {
                jsonrpc: "2.0".to_string(),
                error: ErrorDetail {
                    code: -32601,
                    message: "Method not found".to_string(),
                },
                id: request.id.clone(),
            };
            match serde_json::to_value(error_response) {
                Ok(value) => Json(value),
                Err(_) => create_internal_error(request.id),
            }
        }
    }
}

/// Creates an internal error response
fn create_internal_error(id: serde_json::Value) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "jsonrpc": "2.0",
        "error": {
            "code": -32603,
            "message": "Internal error"
        },
        "id": id
    }))
}

/// Creates the Ethereum RPC endpoint routes
pub fn routes(config: Arc<EthConfig>) -> Router {
    Router::new()
        .route("/", post(json_rpc_handler))
        .with_state(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_eth_chain_id() {
        let config = Arc::new(EthConfig::default());
        let app = routes(config);

        let request_body = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "eth_chainId",
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
        let json_response: JsonRpcResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(json_response.jsonrpc, "2.0");
        assert_eq!(json_response.result, "0x539"); // 1337 in hex
        assert_eq!(json_response.id, 1);
    }

    #[tokio::test]
    async fn test_eth_chain_id_custom_config() {
        let config = Arc::new(EthConfig {
            chain_id: 1,
            chain: Chain::default(),
        });
        let app = routes(config);

        let request_body = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "eth_chainId",
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
        let json_response: JsonRpcResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(json_response.jsonrpc, "2.0");
        assert_eq!(json_response.result, "0x1"); // 1 in hex
        assert_eq!(json_response.id, 42);
    }

    #[tokio::test]
    async fn test_unsupported_method() {
        let config = Arc::new(EthConfig::default());
        let app = routes(config);

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
        let json_response: JsonRpcError = serde_json::from_slice(&body).unwrap();

        assert_eq!(json_response.jsonrpc, "2.0");
        assert_eq!(json_response.error.code, -32601);
        assert_eq!(json_response.error.message, "Method not found");
    }

    #[tokio::test]
    async fn test_eth_block_number() {
        let config = Arc::new(EthConfig::default());
        let app = routes(config);

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
        let json_response: JsonRpcResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(json_response.jsonrpc, "2.0");
        assert_eq!(json_response.result, "0x0"); // Genesis block has number 0
        assert_eq!(json_response.id, 1);
    }

    #[tokio::test]
    async fn test_eth_block_number_with_blocks() {
        use blazeevm_core::block::Block;
        use primitive_types::H256;

        let mut chain = Chain::default();
        // Add block 1
        let block1 = Block::with_header(1, H256::from_low_u64_be(1), H256::zero(), 1000);
        chain.append_block(block1);
        // Add block 2
        let block2 = Block::with_header(2, H256::from_low_u64_be(2), H256::zero(), 2000);
        chain.append_block(block2);

        let config = Arc::new(EthConfig {
            chain_id: 1337,
            chain,
        });
        let app = routes(config);

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
        let json_response: JsonRpcResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(json_response.jsonrpc, "2.0");
        assert_eq!(json_response.result, "0x2"); // Latest block is block 2
        assert_eq!(json_response.id, 1);
    }
}
