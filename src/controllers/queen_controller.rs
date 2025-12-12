use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
    http::StatusCode,
    Json,
};
use sqlx::MySqlPool;

use crate::models::queen::{self, Queen};
use crate::response::{ApiResponse, ApiError};

pub async fn get_all(State(pool): State<MySqlPool>) -> Response {
    match Queen::get_all(&pool).await {
        Ok(hives) => ApiResponse::new(hives, StatusCode::OK).into_response(),
        Err(e) => ApiError::not_found(format!("Database error {:?}", e)),
    }
}
