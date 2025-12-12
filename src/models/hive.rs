use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySqlPool, Result, Error}; // FromRow is a SQLx trait that automatically maps database rows to this struct.
use chrono::{DateTime, Utc};


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Hive {
    #[serde(skip_deserializing)] // Don't expect the id from a form
    pub id: i32,

    pub user_id: i32,
    pub name: String,

    #[serde(skip_deserializing)]
    pub temperature: Option<f32>,

    #[serde(skip_deserializing)]
    pub humidity: Option<f32>,

    pub queen_id: i32,

    #[serde(skip_deserializing)]
    pub created_at: Option<DateTime<Utc>>,

    #[serde(skip_deserializing)]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateSensorData {
    pub temperature: Option<f32>,
    pub humidity: Option<f32>,
}

impl Hive {
    // This is a static method so no self
    // Returns either a Vec of hives or a database error

    // The underscore _ allows SQLx to infer the row type automatically based on the query result,
    // while `Hive` is the target type to which each row will be mapped. 
    // This means SQLx will automatically map the columns of the query result to the fields in the `Hive` struct.
    pub async fn get_all(pool: &MySqlPool) -> Result<Vec<Hive>> {
        sqlx::query_as::<_, Hive>(
            "SELECT *
            FROM hives"
        )
        .fetch_all(pool)
        .await
    }

    pub async fn add(pool: &MySqlPool, data: Hive ) -> Result<bool> {
        const QUERY: &str = "
            INSERT INTO hives (user_id, name, queen_id)
            VALUES(?, ?, ?);
        ";
        sqlx::query(QUERY)
            .bind(data.user_id)
            .bind(data.name)
            .bind(data.queen_id)
            .execute(pool)
            .await?;

        Ok(true)
    }

    pub async fn update_sensor_data(pool: &MySqlPool,
        temperature: Option<f32>,
        humidity: Option<f32>,
        id: i32
    ) -> Result<bool, Error> {
        const QUERY: &str = "
            UPDATE hives
            SET temperature = ?, humidity = ?
            WHERE id = ?;
        ";

        let result = sqlx::query(QUERY)
            .bind(temperature)
            .bind(humidity)
            .bind(id)
            .execute(pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn find(pool: &MySqlPool, id: i32) -> Result<Option<Hive>> {
        sqlx::query_as::<_, Hive>(
            "SELECT *
            FROM hives
            WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(pool) // We use optional because it just returns None if the row is not found
        .await
    }

    pub async fn delete(pool: &MySqlPool, id: i32) -> bool {
        let query = "
            DELETE FROM hives
            WHERE id = ?;
        ";
        let result = sqlx::query(query)
            .bind(id)
            .fetch_optional(pool)
            .await;

        if result.is_err() {
            return true;
        }

        return true;
    }

    pub async fn delete_all(pool: &MySqlPool) -> Result<bool, Error> {
        const QUERY: &str = "
            DELETE FROM hives;
        ";
        let result = sqlx::query(QUERY)
            .execute(pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}
