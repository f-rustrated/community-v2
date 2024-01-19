mod account;
mod response;

use axum::{Router, routing::get};
use account::account_router; 

pub async fn root() -> &'static str {
    "Hello world"
}

pub fn router() -> Router {
    Router::new()
        .route("/", get(root))
        .nest("/api/v1/accounts", account_router())
}