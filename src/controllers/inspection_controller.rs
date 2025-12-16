use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
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

pub async fn add(State(pool): State<MySqlPool>, Form(data): Form<Inspection>) -> Response {
    match Inspection::add(&pool, data).await {
        Ok(created_inspection) => ApiResponse::created(created_inspection).into_response(),
        Err(e) => ApiError::internal_error(format!("Database error: {:?}", e)),
    }
}

pub async fn get_all_from_hive(State(pool): State<MySqlPool>, Path(id): Path<i32>) -> Response {
    match Inspection::get_from_hive(&pool, id).await {
        Ok(inspections) => {
            if inspections.is_empty() {
                return ApiError::not_found("Could not find any inspections from this hive");
            }
            ApiResponse::new(inspections, StatusCode::OK).into_response()
        }
        Err(e) => ApiError::internal_error(format!("Database error: {:?}", e)),
    }
}

pub async fn update(State(pool): State<MySqlPool>, Path(id): Path<i32>, Form(data): Form<Inspection>) -> Response {
    match Inspection::update(&pool, id, data).await {
        Ok(true) => ApiResponse::new(true, StatusCode::OK).into_response(),
        Ok(false) => ApiError::not_found("not found"),
        Err(e) => ApiError::internal_error(format!("Database error: {:?}", e)),
    }
}
