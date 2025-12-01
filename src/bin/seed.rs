/// Database Seeder
/// This is a separate binary (in src/bin/) so it doesn't affect
/// the main API binary

use sqlx::mysql::MySqlPoolOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv::dotenv().ok();

    println!("Starting database seeding...");

    // Get database URL
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not found");

    // Connect to database
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    println!("Clearing existing data...");
    
    // Clear existing data
    sqlx::query("DELETE FROM hives")
        .execute(&pool)
        .await?;

    println!("Inserting seed data...");

    // Insert test hives
    let hives = vec![
        ("Hive 1", Some(35.5), Some(69.4)),
        ("Hive 2", Some(38.10), Some(57.2)),
        ("Hive 3", Some(32.7), Some(64.15)),
        ("Hive 4", Some(30.3), Some(67.5)),
    ];

    for (name, temperature, humidity) in hives {
        sqlx::query(
            "INSERT INTO hives (name, temperature, humidity) 
             VALUES (?, ?, ?)"
        )
        .bind(name)
        .bind(temperature)
        .bind(humidity)
        .execute(&pool)
        .await?;
        
        println!("Added: {}", name);
    }

    println!("Seeding complete");
    Ok(())
}
