use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
    Form,
};
use sqlx::MySqlPool;

use crate::{models::hive::{Hive, UpdateSensorData}, response::ApiError};
use crate::response::ApiResponse;

pub async fn get_all(State(pool): State<MySqlPool>) -> Response {
    match Hive::get_all(&pool).await {
        Ok(hives) => {
            ApiResponse::new(hives, StatusCode::OK).into_response()
        }
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            // Err(StatusCode::INTERNAL_SERVER_ERROR)
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body("Internal server error".into())
                .unwrap()
        }
    }
}

pub async fn add(State(pool): State<MySqlPool>, Form(data): Form<Hive>) -> Result<Response, StatusCode> {
    match Hive::add(&pool, data).await {
        Ok(_) => Ok((StatusCode::CREATED, "Hive created successfully").into_response()),
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
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
        Ok(None) => ApiError::not_found("Not found"),
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            // Err(StatusCode::INTERNAL_SERVER_ERROR)
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body("Internal server error".into())
                .unwrap()
        }
    }
}

pub async fn delete(State(pool): State<MySqlPool>, Path(id): Path<i32>) -> Result<Json<bool>, StatusCode> {
    if Hive::delete(&pool, id).await == false {
        return Err(StatusCode::NOT_FOUND);
    }
    return Ok(Json(true));
}
