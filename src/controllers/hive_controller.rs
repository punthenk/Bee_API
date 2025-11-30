use axum::{
    extract::{State},
    http::StatusCode,
    Json,
};
use sqlx::MySqlPool;

use crate::models::hive::Hive;

pub async fn get_all_hives(State(pool): State<MySqlPool>) -> Result<Json<Vec<Hive>>, StatusCode> {
    match Hive::get_all(&pool).await {
        Ok(hives) => Ok(Json(hives)),
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
