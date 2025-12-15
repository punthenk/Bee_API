use axum::{routing::get, routing::post, routing::delete, routing::patch, Router};
use sqlx::MySqlPool;

use crate::controllers::hive_controller;
use crate::controllers::queen_controller;
use crate::controllers::inspection_controller;

pub fn create_routes(pool: MySqlPool) -> Router {
    Router::new()
        // GET
        .route("/hives", get(hive_controller::get_all))
        .route("/hive/{id}", get(hive_controller::find))
        .route("/queens", get(queen_controller::get_all))
        .route("/inspections", get(inspection_controller::get_all))
        .route("/inspection/{id}", get(inspection_controller::find))

        // POST
        .route("/hive", post(hive_controller::add))
        .route("/inspection", post(inspection_controller::add))

        // PATCH
        .route("/hive/{id}/sensor", patch(hive_controller::update_sensor_data))

        // DELETE
        .route("/hive/{id}", delete(hive_controller::delete))
        .with_state(pool)
}
