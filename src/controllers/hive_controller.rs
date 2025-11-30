use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::MySqlPool;

use crate::models::hive::Hive;

pub async fn get_all(State(pool): State<MySqlPool>) -> Result<Json<Vec<Hive>>, StatusCode> {
    match Hive::get_all(&pool).await {
        Ok(hives) => Ok(Json(hives)),
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn find(State(pool): State<MySqlPool>, Path(id): Path<i32>) -> Result<Json<Hive>, StatusCode> {
    match Hive::find(&pool, id).await {
        Ok(Some(hive)) => Ok(Json(hive)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
