use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
    Form,
};
use sqlx::MySqlPool;

use crate::models::hive::{Hive, UpdateSensorData};
use crate::response::{ApiResponse, ApiError};

pub async fn get_all(State(pool): State<MySqlPool>) -> Response {
    match Hive::get_all(&pool).await {
        Ok(hives) => ApiResponse::new(hives, StatusCode::OK).into_response(),
        Err(e) => ApiError::internal_error(format!("Database error: {:?}", e)),
    }
}

pub async fn add(State(pool): State<MySqlPool>, Form(data): Form<Hive>) -> Response {
    match Hive::add(&pool, data).await {
        Ok(created_hive) => ApiResponse::created(created_hive).into_response(),
        Err(e) => ApiError::internal_error(format!("Database error: {:?}", e)),
    }
}

pub async fn update_sensor_data(State(pool): State<MySqlPool>, 
    Path(id): Path<i32>,
    Json(data): Json<UpdateSensorData>
) -> Response {
    match Hive::update_sensor_data(&pool, data.temperature, data.humidity, id).await {
        Ok(true) => ApiResponse::new(data, StatusCode::OK).into_response(),
        Ok(false) => ApiError::not_found("Hive could not be found"),
        Err(e) => ApiError::internal_error(format!("Database error: {:?}", e)),
    }
}

pub async fn find(State(pool): State<MySqlPool>, Path(id): Path<i32>) -> Response {
    match Hive::find(&pool, id).await {
        Ok(Some(hive)) => ApiResponse::new(hive, StatusCode::FOUND).into_response(),
        Ok(None) => ApiError::not_found("Hive not found").into_response(),
        Err(e) => ApiError::internal_error(format!("Database error {:?}", e)).into_response(),
    }
}

pub async fn delete(State(pool): State<MySqlPool>, Path(id): Path<i32>) -> Response {
    let result = Hive::delete(&pool, id).await;

    match result {
        Ok(true) => (axum::http::StatusCode::NO_CONTENT, ()).into_response(),
        Ok(false) => ApiError::not_found("Hive not found"),
        Err(e) => ApiError::internal_error(format!("Database error: {}", e)),
    }
}
