use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySqlPool, Result, Error}; // FromRow is a SQLx trait that automatically maps database rows to this struct.
use chrono::{DateTime, Utc};


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    #[serde(skip_deserializing)] // Don't expect the id from a form
    pub id: i32,

    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub password: String,

    #[serde(skip_deserializing)]
    pub created_at: Option<DateTime<Utc>>,

    #[serde(skip_deserializing)]
    pub updated_at: Option<DateTime<Utc>>,
}

impl User {
    pub async fn add(pool: &MySqlPool, data: User ) -> Result<bool> {
        const QUERY: &str = "
            INSERT INTO users (firstname, lastname, email, password)
            VALUES(?, ?, ?, ?);
        ";
        sqlx::query(QUERY)
            .bind(data.firstname)
            .bind(data.lastname)
            .bind(data.email)
            .bind(data.password)
            .execute(pool)
            .await?;

        Ok(true)
    }

    pub async fn delete_all(pool: &MySqlPool) -> Result<bool, Error> {
        const QUERY: &str = "
            DELETE FROM users;
        ";
        let result = sqlx::query(QUERY)
            .execute(pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}
