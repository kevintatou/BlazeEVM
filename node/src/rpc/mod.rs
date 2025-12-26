use axum::Router;

pub mod eth;
mod health;

/// Creates and returns the router with all RPC endpoints
pub fn routes(chain: eth::AppState) -> Router {
    Router::new()
        .merge(health::routes())
        .nest("/eth", eth::routes(chain))
}
