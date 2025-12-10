use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub api_version: String,
    pub api_name: String,
    pub count: usize,
    pub status: u16,
    pub status_message: String,
    pub data: T,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn new(data: T, status: StatusCode) -> Self {
        let count = Self::count_items(&data);

        Self {
            api_version: "1.0".to_string(),
            api_name: "bee_api".to_string(),
            count,
            status: status.as_u16(),
            status_message: Self::status_text(status),
            data,
        }
    }

    fn status_text(status: StatusCode) -> String {
        status.canonical_reason()
            .unwrap_or("Unknown")
            .to_string()
    }

    fn count_items(data: &T) -> usize {
        if let Ok(json) = serde_json::to_value(data) {
            if let Some(arr) = json.as_array() {
                return arr.len();
            }
        }
        1
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.status)
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

        (status, Json(self)).into_response()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorData {
    pub error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

impl ErrorData {
    pub fn new(error: impl Into<String>) -> Self {
        Self {
            error: error.into(),
            details: None,
        }
    }
}

pub type ApiError = ApiResponse<ErrorData>;

impl ApiError {
    pub fn not_found(message: impl Into<String>) -> Response {
        Self::new(
            ErrorData::new(message),
            StatusCode::NOT_FOUND
        ).into_response()
    }
}
