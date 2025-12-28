use axum::Router;
use std::sync::Arc;

mod eth;
mod health;

pub use eth::EthConfig;

/// Creates and returns the router with all RPC endpoints
pub fn routes() -> Router {
    let eth_config = Arc::new(EthConfig::default());
    Router::new()
        .merge(health::routes())
        .merge(eth::routes(eth_config))
}
