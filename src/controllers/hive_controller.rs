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
        Ok(hives) => {
            ApiResponse::new(hives, StatusCode::OK).into_response()
        }
        Err(e) => ApiError::internal_error(format!("Database error: {:?}", e)),
    }
}

pub async fn add(State(pool): State<MySqlPool>, Form(data): Form<Hive>) -> Response {
    match Hive::add(&pool, data.clone()).await {
        Ok(_) => ApiResponse::created(data).into_response(),
        Err(e) => ApiError::internal_error(format!("Database error: {:?}", e)),
    }
}

pub async fn update_sensor_data(State(pool): State<MySqlPool>, 
    Path(id): Path<i32>,
    Json(data): Json<UpdateSensorData>
) -> Result<Response, StatusCode> {
    match Hive::update_sensor_data(&pool, data.temperature, data.humidity, id).await {
        Ok(true) => Ok((StatusCode::OK, "Hive updated successfully").into_response()),
        Ok(false) => Ok((StatusCode::NOT_FOUND, "Hive could not be found").into_response()),
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn find(State(pool): State<MySqlPool>, Path(id): Path<i32>) -> Response {
    match Hive::find(&pool, id).await {
        Ok(Some(hive)) => ApiResponse::new(hive, StatusCode::FOUND).into_response(),
        Ok(None) => ApiError::not_found("Hive not found").into_response(),
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            ApiError::internal_error(format!("Database error {:?}", e)).into_response()
        }
    }
}

pub async fn delete(State(pool): State<MySqlPool>, Path(id): Path<i32>) -> Result<Json<bool>, StatusCode> {
    if Hive::delete(&pool, id).await == false {
        return Err(StatusCode::NOT_FOUND);
    }
    return Ok(Json(true));
}
