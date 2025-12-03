use axum::{routing::get, Router};
use sqlx::MySqlPool;

use crate::controllers::hive_controller;
use crate::controllers::queen_controller;

pub fn create_routes(pool: MySqlPool) -> Router {
    Router::new()
        // GET
        .route("/hives", get(hive_controller::get_all))
        .route("/hive/{id}", get(hive_controller::find))
        .route("/queens/", get(queen_controller::get_all))
        .with_state(pool)
}
