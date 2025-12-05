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
    let mut hives: Vec<(String, Option<f32>, Option<f32>)> = Vec::new();
    let hive_count = 15;

    for i in 1..=hive_count {
        let hive_name = format!("Hive {i}");
        hives.push((hive_name, Some(35.5), Some(65.6)));
    }

    for (name, temperature, humidity) in hives {
        sqlx::query(
            "INSERT INTO hives (name, temperature, humidity) 
             VALUES (?, ?, ?)"
        )
        .bind(&name)
        .bind(temperature)
        .bind(humidity)
        .execute(&pool)
        .await?;
        
        println!("Added: {}", name);
    }

    println!("Seeding complete");
    Ok(())
}
