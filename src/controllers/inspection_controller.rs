use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
    Form,
};
use sqlx::MySqlPool;

use crate::models::inspection::{Inspection};
use crate::response::{ApiResponse, ApiError};

pub async fn get_all(State(pool): State<MySqlPool>) -> Response {
    match Inspection::get_all(&pool).await {
        Ok(inspections) => ApiResponse::new(inspections, StatusCode::OK).into_response(),
        Err(e) => ApiError::internal_error(format!("Database error: {:?}", e)),
    }
}

pub async fn find(State(pool): State<MySqlPool>, Path(id): Path<i32>) -> Response {
    match Inspection::find(&pool, id).await {
        Ok(Some(inspection)) => ApiResponse::new(inspection, StatusCode::OK).into_response(),
        Ok(None) => ApiError::not_found("Inspection not found"),
        Err(e) => ApiError::internal_error(format!("Database error: {:?}", e)),
    }
}
