/// Database Seeder
/// This is a separate binary (in src/bin/) so it doesn't affect
/// the main API binary

use sqlx::mysql::MySqlPoolOptions;

use beekeeper_API::models::hive::Hive;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    println!("Starting database seeding...");

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not found");

    // Connect to database
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    println!("Clearing existing data...");
    
    // Clear all existing data
    Hive::delete_all(&pool).await?;

    println!("Inserting seed data...");

    // Insert test hives
    let hive_count = 15;

    for i in 1..=hive_count {
        let hive = Hive {
            id: 1, // NOT USED IN QUERY
            user_id: 1,
            queen_id: 1,
            name: format!("Hive {}", i),
            temperature: None, // NOT USED IN QUERY
            humidity: None, // NOT USED IN QUERY
            created_at: None, // NOT USED IN QUERY
            updated_at: None, // NOT USED IN QUERY
        };

        Hive::add(&pool, hive).await.expect("Failed to add hive");
        println!("Added: Hive {}", i);
    }

    println!("Seeding complete");
    Ok(())
}
