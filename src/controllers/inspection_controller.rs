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

pub async fn find(State(pool): State<MySqlPool>, Path(id): Path<i32>) -> Result<Json<Inspection>, StatusCode> {
    match Inspection::find(&pool, id).await {
        Ok(Some(inspection)) => Ok(Json(inspection)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
