use axum::{
    extract::{State},
    response::{IntoResponse, Response},
};
use sqlx::MySqlPool;

use crate::models::queen::{Queen};
use crate::response::{ApiResponse, ApiError};

pub async fn get_all(State(pool): State<MySqlPool>) -> Response {
    match Queen::get_all(&pool).await {
        Ok(queens) => ApiResponse::ok(queens),
        Err(e) => ApiError::internal_error(e.to_string()),
    }
}
