use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::MySqlPool;

use crate::models::queen::{self, Queen};

pub async fn get_all(State(pool): State<MySqlPool>) -> Result<Json<Vec<Queen>>, StatusCode> {
    match Queen::get_all(&pool).await {
        Ok(hives) => Ok(Json(hives)),
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
