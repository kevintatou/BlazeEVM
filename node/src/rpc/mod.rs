use axum::Router;

mod health;

/// Creates and returns the router with all RPC endpoints
pub fn routes() -> Router {
    Router::new().merge(health::routes())
}
