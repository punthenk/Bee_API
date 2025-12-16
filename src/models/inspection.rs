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
    pub date: Option<NaiveDate>,
    pub behaviour: Option<String>,
    pub queen_seen: Option<bool>,
    pub honeycomb_count: Option<i32>,
    pub windows_occupied: Option<i32>,
    pub BRIAS: Option<String>,
    pub BRIAS_healthy: Option<String>,
    pub invested_swarm_cells: Option<i32>,
    pub stock_food: Option<i32>,
    pub pollen: Option<i32>,
    pub mite_fall: Option<i32>,

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

    pub async fn get_from_hive(pool: &MySqlPool, id: i32) -> Result<Vec<Inspection>> {
        const QUERY: &str = "
            SELECT * FROM
            inspections
            WHERE hive_id = ?;
        ";
        sqlx::query_as::<_, Inspection>(QUERY)
            .bind(id)
            .fetch_all(pool)
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

    pub async fn update(pool: &MySqlPool, id: i32, data: Inspection) -> Result<bool> {
        let mut query_builder = sqlx::QueryBuilder::new("UPDATE inspections SET ");
        let mut has_updates = false;

        macro_rules! add {
            ($field_name:expr, $field_value:expr) => {
                    if let Some(ref val) = $field_value {
                    if has_updates {
                    query_builder.push(", ");
                }
                query_builder.push(format!("{} = ", $field_name));
                query_builder.push_bind(val);
                has_updates = true;
                }
            };
        }

        add!("date", data.date);
        add!("behaviour", data.behaviour);
        add!("queen_seen", data.queen_seen);
        add!("honeycomb_count", data.honeycomb_count);
        add!("windows_occupied", data.windows_occupied);
        add!("BRIAS", data.BRIAS);
        add!("BRIAS_healthy", data.BRIAS_healthy);
        add!("invested_swarm_cells", data.invested_swarm_cells);
        add!("stock_food", data.stock_food);
        add!("pollen", data.pollen);
        add!("mite_fall", data.mite_fall);

        if !has_updates {
            return Ok(false);
        }

        query_builder.push(", updated_at = ").push_bind(Utc::now());
        query_builder.push(" WHERE id = ").push_bind(id);

        let result = query_builder.build().execute(pool).await?;
        Ok(result.rows_affected() > 0)
    }
}
