use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CalculateRequest {
    pub expression: String,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
pub enum CalculateResponse {
    Success {
        result: f64,
    },
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HistoryRequest {
    pub limit: u16,
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
