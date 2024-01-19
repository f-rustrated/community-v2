use crate::router::router;
mod router;


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = router();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap()
}