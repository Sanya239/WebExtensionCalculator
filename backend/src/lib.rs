use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CalculateRequest {
    pub expression: String,
    pub device_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
pub enum CalculateResponse {
    Success { result: f64 },
    Error {
        error_type: CalculationErrorType,
        message: String,
        position: u32,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CalculationErrorType {
    ParseError,
    EvaluationError,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HistoryRequest {
    pub limit: u16,
    pub device_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(transparent)]
pub struct HistoryResponse(pub Vec<HistoryEntryResponse>);

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
pub enum HistoryEntryResponse {
    Success {
        expression: String,
        result: f64,
        timestamp: DateTime<Utc>,
    },
    Error {
        expression: String,
        error_type: CalculationErrorType,
        message: String,
        position: u32,
        timestamp: DateTime<Utc>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ApiErrorResponse {
    pub error_type: ApiErrorType,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ApiErrorType {
    InvalidRequest,
    InternalError,
}

#[derive(Debug)]
pub enum ApiError {
    BadRequest(String),
    Internal(String),
}

impl ApiError {
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::BadRequest(message.into())
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal(message.into())
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_type, message) = match self {
            ApiError::BadRequest(m) => (
                StatusCode::BAD_REQUEST,
                ApiErrorType::InvalidRequest,
                m,
            ),
            ApiError::Internal(m) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ApiErrorType::InternalError,
                m,
            ),
        };
        (status, Json(ApiErrorResponse { error_type, message })).into_response()
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> Self {
        tracing::error!(error = %err, "database error");
        ApiError::internal(format!("database error: {err}"))
    }
}