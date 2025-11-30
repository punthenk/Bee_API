use axum::{routing::get, Router};
use sqlx::MySqlPool;

use crate::controllers::hive_controller;

pub fn create_routes(pool: MySqlPool) -> Router {
    Router::new()
        .route("/hives", get(hive_controller::get_all))
        .route("/hive/{id}", get(hive_controller::find))
        .with_state(pool)
}
