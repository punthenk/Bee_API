use axum::{
    extract::{State},
    response::{IntoResponse, Response},
    http::StatusCode,
};
use sqlx::MySqlPool;

use crate::models::queen::{Queen};
use crate::response::{ApiResponse, ApiError};

pub async fn get_all(State(pool): State<MySqlPool>) -> Response {
    match Queen::get_all(&pool).await {
        Ok(queens) => ApiResponse::new(queens, StatusCode::OK).into_response(),
        Err(e) => ApiError::internal_error(format!("Database error: {:?}", e)),
    }
}
