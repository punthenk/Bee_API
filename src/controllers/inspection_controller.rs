use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
    Form,
};
use sqlx::MySqlPool;

use crate::models::inspection::{Inspection};

pub async fn get_all(State(pool): State<MySqlPool>) -> Result<Json<Vec<Inspection>>, StatusCode> {
    match Inspection::get_all(&pool).await {
        Ok(inspections) => Ok(Json(inspections)),
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
