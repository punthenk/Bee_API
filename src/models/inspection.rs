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
    pub BRIAS_healty: String,
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
}
