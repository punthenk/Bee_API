use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::MySqlPool;

use crate::models::hive::Hive;

pub async fn get_all_hives( State(pool): State<MySqlPool>,) -> Result<Json<Vec<Hive>>, StatusCode> {
    // Query all hives from the database
    // The `query_as!` macro does compile-time SQL verification (amazing!)
    // and automatically maps the result to our Hive struct
    let hives = sqlx::query_as::<_, Hive>(
        "SELECT *
         FROM hives"
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        // Log the error for debugging
        eprintln!("Database error: {:?}", e);
        // Return 500 Internal Server Error to the client
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Wrap the result in Json() to automatically serialize to JSON
    Ok(Json(hives))
}
