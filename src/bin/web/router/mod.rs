use axum::{Router, routing::get};

pub async fn root() -> &'static str {
    "Hello world"
}

pub fn router() -> Router {
    Router::new()
        .route("/", get(root))
}