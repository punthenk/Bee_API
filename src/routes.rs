use axum::{routing::get, routing::post, routing::delete, Router};
use sqlx::MySqlPool;

use crate::controllers::hive_controller;
use crate::controllers::queen_controller;

pub fn create_routes(pool: MySqlPool) -> Router {
    Router::new()
        // GET
        .route("/hives", get(hive_controller::get_all))
        .route("/hive/{id}", get(hive_controller::find))
        .route("/queens/", get(queen_controller::get_all))

        // POST
        .route("/hive", post(hive_controller::add))

        // DELETE
        .route("/hive/{id}", delete(hive_controller::delete))
        .with_state(pool)
}
