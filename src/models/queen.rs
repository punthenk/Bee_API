use serde::{Serialize, Deserialize};
use sqlx::{FromRow, MySqlPool, Result, Error}; // FromRow is a SQLx trait that automatically maps database rows to this struct.
use chrono::{DateTime, Utc};


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Queen {
    pub id: i32,
    pub race: String,
    pub origin: Option<String>,
    pub birth_year: i32,
    pub fertilization_site: Option<String>,
    pub clipped: bool,
    pub created_at: Option<DateTime<Utc>>,
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

    pub async fn add(pool: &MySqlPool, data: Queen ) -> Result<bool> {
        const QUERY: &str = "
            INSERT INTO queens (race, origin, birth_year, fertilization_site, clipped)
            VALUES(?, ?, ?, ?, ?);
        ";
        sqlx::query(QUERY)
            .bind(data.race)
            .bind(data.origin)
            .bind(data.birth_year)
            .bind(data.fertilization_site)
            .bind(data.clipped)
            .execute(pool)
            .await?;

        Ok(true)
    }

    pub async fn delete_all(pool: &MySqlPool) -> Result<bool, Error> {
        const QUERY: &str = "
            DELETE FROM queens;
        ";
        let result = sqlx::query(QUERY)
            .execute(pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

}
