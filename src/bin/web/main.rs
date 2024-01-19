use dotenv::dotenv;
use community_v2::config::config;

use crate::router::router;

mod router;


// pub mod.rs composition_root;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv().ok();

    let app = router();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    println!("{:?}", config());
    axum::serve(listener, app).await.unwrap()
}