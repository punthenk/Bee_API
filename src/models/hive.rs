use serde::{Serialize, Deserialize};
use sqlx::{FromRow}; // FromRow is a SQLx trait that automatically maps database rows to this struct.
use chrono;


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Hive {
    pub id: i32,
    pub name: String,
    pub temperature: Option<f32>,
    pub humidity: Option<f32>,
}
