use axum::{routing::get, Router};
use sqlx::MySqlPool;

use crate::controllers::hive_controller;

pub fn create_routes(pool: MySqlPool) -> Router {
    Router::new()
        .route("/hives", get(hive_controller::get_all_hives))
        .with_state(pool)
}
