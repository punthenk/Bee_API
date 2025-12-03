use serde::{Serialize, Deserialize};
use sqlx::{FromRow, MySqlPool, Result, pool}; // FromRow is a SQLx trait that automatically maps database rows to this struct.
use chrono::{DateTime, Utc};


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Queen {
    pub id: i32,
    pub race: String,
    pub origin: Option<String>,
    pub birthday: DateTime<Utc>,
    pub fertilization_site: Option<String>,
    pub clipped: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Queen {
    pub async fn get_all(pool: &MySqlPool) -> Result<Vec<Queen>> {
        sqlx::query_as::<_, Queen>(
            "SELECT *
            FROM queens"
        )
        .fetch_all(pool)
        .await
    }
}
