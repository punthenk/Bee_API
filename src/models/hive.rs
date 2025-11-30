use serde::{Serialize, Deserialize};
use sqlx::{FromRow, MySqlPool, Result}; // FromRow is a SQLx trait that automatically maps database rows to this struct.
use chrono::{DateTime, Utc};


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Hive {
    pub id: i32,
    pub name: String,
    pub temperature: Option<f32>,
    pub humidity: Option<f32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Hive {
    // This is static method so no self
    // Returns either a Vec of hives or a database error
    pub async fn get_all(pool: &MySqlPool) -> Result<Vec<Hive>> {
        sqlx::query_as::<_, Hive>(
            "SELECT *
            FROM hives"
        )
        .fetch_all(pool)
        .await
    }
}
