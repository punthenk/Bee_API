use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySqlPool, Result, Error}; // FromRow is a SQLx trait that automatically maps database rows to this struct.
use chrono::{DateTime, NaiveDate, Utc};


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Inspection {
    #[serde(skip_deserializing)] // Don't expect the id from a form
    pub id: i32,

    pub user_id: i32,
    pub hive_id: i32,
    pub queen_id: i32,
    pub date: NaiveDate,
    pub behaviour: String,
    pub queen_seen: bool,
    pub honeycomb_count: i32,
    pub windows_occupied: i32,
    pub BRIAS: String,
    pub BRIAS_healthy: String,
    pub invested_swarm_cells: i32,
    pub stock_food: i32,
    pub pollen: i32,
    pub mite_fall: i32,

    #[serde(skip_deserializing)]
    pub created_at: Option<DateTime<Utc>>,

    #[serde(skip_deserializing)]
    pub updated_at: Option<DateTime<Utc>>,
}

impl Inspection {
    pub async fn get_all(pool: &MySqlPool) -> Result<Vec<Inspection>> {
        const QUERY: &str = "
            SELECT * FROM
            inspections
        ";

        sqlx::query_as::<_, Inspection>(QUERY)
            .fetch_all(pool)
            .await
    }

    pub async fn find(pool: &MySqlPool, id: i32) -> Result<Option<Inspection>> {
        sqlx::query_as::<_, Inspection>(
            "SELECT *
            FROM inspections
            WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(pool) // We use optional because it just returns None if the row is not found
        .await
    }

    pub async fn add(pool: &MySqlPool, data: Inspection) -> Result<bool> {
        const QUERY: &str = "
            INSERT INTO inspections (
                user_id, 
                hive_id, 
                queen_id,
                date,
                behaviour,
                queen_seen,
                honeycomb_count,
                windows_occupied,
                BRIAS,
                BRIAS_healthy,
                invested_swarm_cells,
                stock_food,
                pollen,
                mite_fall
            )
            VALUES(?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);
        ";
        sqlx::query(QUERY)
            .bind(data.user_id)
            .bind(data.hive_id)
            .bind(data.queen_id)
            .bind(data.date)
            .bind(data.behaviour)
            .bind(data.queen_seen)
            .bind(data.honeycomb_count)
            .bind(data.windows_occupied)
            .bind(data.BRIAS)
            .bind(data.BRIAS_healthy)
            .bind(data.invested_swarm_cells)
            .bind(data.stock_food)
            .bind(data.pollen)
            .bind(data.mite_fall)
            .execute(pool)
            .await?;

        Ok(true)
    }
}
