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
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let database_url = if let Ok(url) = std::env::var("DATABASE_URL") {
        url
    } else {
        None.expect("Did not found it")
    };

    println!("Connecting to db...");

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    println!("Is connected to db");

    let app = routes::create_routes(pool);

    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let address = format!("{}:{}", host, port);

    let listener = tokio::net::TcpListener::bind(&address)
        .await
        .expect("Failed to bind to address");

    println!("Listening on http://{}", address);

    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}
