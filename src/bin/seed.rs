/// Database Seeder
/// This is a separate binary (in src/bin/) so it doesn't affect
/// the main API binary

use sqlx::mysql::MySqlPoolOptions;

use beekeeper_api::models::hive::Hive;
use beekeeper_api::models::queen::Queen;
use beekeeper_api::models::user::User;

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
    Queen::delete_all(&pool).await?;
    User::delete_all(&pool).await?;

    println!("Inserting seed data...");

    let user = User {
        id: 1,
        firstname: "admin".to_string(),
        lastname: "admin".to_string(),
        email: "admin@admin.com".to_string(),
        password: "password".to_string(),
        created_at: None, // NOT USED IN QUERY
        updated_at: None, // NOT USED IN QUERY
    };
    User::add(&pool, user).await.expect("Failed to add user");
    println!("Added: User");


    // Insert test queens
    let queen_count = 4;

    for i in 1..=queen_count {
        let queen = Queen {
            id: 1, // NOT USED IN QUERY
            race: "Race".to_string(),
            origin: Some("Nature".to_string()),
            birth_year: 2025,
            fertilization_site: None,
            clipped: true,
            created_at: None, // NOT USED IN QUERY
            updated_at: None, // NOT USED IN QUERY
        };

        Queen::add(&pool, queen).await.expect("Failed to add queen");
        println!("Added: Queen {}", i);
    }

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
