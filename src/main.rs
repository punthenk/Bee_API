use axum::{routing::get, Router, Json};
use serde::Serialize;
use std::net::SocketAddr;

#[derive(Serialize)]
struct ApiResponse {
    message: String,
    status: String,
}

async fn hello_handler() -> Json<ApiResponse> {
    Json(ApiResponse {
        message: "Hello, the API is working".to_string(),
        status: "success".to_string(),
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello_handler));

    // Run it on localhost:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();
    
    axum::serve(listener, app)
        .await
        .unwrap();
}
