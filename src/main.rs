use std::net::SocketAddr;
use sqlx::mysql::MySqlPoolOptions;
use tracing_subscriber;

mod controllers {
    pub mod hive_controller;
}

mod models {
    pub mod hive;
}
mod routes;

/// This function:
/// 1. Sets up logging
/// 2. Connects to the database
/// 3. Creates the router with all routes
/// 4. Starts the HTTP server

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "mysql://admin:root@localhost/beekeeper_db".to_string());

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    let app = routes::create_routes(pool);

    // Run it on localhost:3000
    let address = SocketAddr::from(([127, 0, 0, 1], 3000));

    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("Failed to bind to adress");

    println!("Server running on http://{}", address);
    
    // Start the server
    // This will run until it is killed
    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}
