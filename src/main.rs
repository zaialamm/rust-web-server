use axum::{
    response::Json,
    routing::get,
    Router,
};

use serde::Serialize;
use tokio::net::TcpListener;

#[derive(Serialize)]
struct Message {
    message: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health));

    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to port");
    
    println!("Server running on http://0.0.0.0:3000");
    
    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}

async fn root() -> Json<Message> {
    Json(Message {
        message: "Hello, Axum!".to_string(),
    })
}

async fn health() -> Json<Message> {
    Json(Message {
        message: "Server is healthy!".to_string(),
    })
}