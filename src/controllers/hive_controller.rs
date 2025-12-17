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
        Ok(hives) => ApiResponse::ok(hives),
        Err(e) => ApiError::internal_error(e.to_string()),
    }
}

pub async fn add(State(pool): State<MySqlPool>, Form(data): Form<Hive>) -> Response {
    match Hive::add(&pool, data).await {
        Ok(created_hive) => ApiResponse::created(created_hive),
        Err(e) => ApiError::internal_error(e.to_string()),
    }
}

pub async fn update_sensor_data(State(pool): State<MySqlPool>, 
    Path(id): Path<i32>,
    Json(data): Json<UpdateSensorData>
) -> Response {
    match Hive::update_sensor_data(&pool, data.temperature, data.humidity, id).await {
        Ok(true) => ApiResponse::ok(data),
        Ok(false) => ApiError::not_found("Hive could not be found"),
        Err(e) => ApiError::internal_error(e.to_string()),
    }
}

pub async fn find(State(pool): State<MySqlPool>, Path(id): Path<i32>) -> Response {
    match Hive::find(&pool, id).await {
        Ok(Some(hive)) => ApiResponse::ok(hive),
        Ok(None) => ApiError::not_found("Hive not found"),
        Err(e) => ApiError::internal_error(e.to_string()),
    }
}

pub async fn delete(State(pool): State<MySqlPool>, Path(id): Path<i32>) -> Response {
    let result = Hive::delete(&pool, id).await;

    match result {
        Ok(true) => (StatusCode::NO_CONTENT, ()).into_response(),
        Ok(false) => ApiError::not_found("Hive not found"),
        Err(e) => ApiError::internal_error(e.to_string()),
    }
}
