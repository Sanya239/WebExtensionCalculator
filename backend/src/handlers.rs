use axum::{
    Json,
    extract::{Query, State},
};
use calculator_backend::{
    ApiError, CalculateRequest, CalculateResponse, CalculationErrorType, HistoryEntryResponse,
    HistoryRequest, HistoryResponse,
};
use calculator_core::CalculationErrorKind;

use crate::{AppState, repo};

const DEVICE_ID_MAX_LEN: usize = 128;

fn validate_device_id(device_id: &str) -> Result<(), ApiError> {
    if device_id.is_empty() {
        return Err(ApiError::bad_request("device_id must not be empty"));
    }
    if device_id.len() > DEVICE_ID_MAX_LEN {
        return Err(ApiError::bad_request(format!(
            "device_id must be at most {DEVICE_ID_MAX_LEN} characters long"
        )));
    }
    if !device_id
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    {
        return Err(ApiError::bad_request(
            "device_id may contain only ASCII letters, digits, '-' and '_'",
        ));
    }
    Ok(())
}

/// POST /api/calculate
pub async fn calculate(
    State(state): State<AppState>,
    Json(req): Json<CalculateRequest>,
) -> Result<Json<CalculateResponse>, ApiError> {
    validate_device_id(&req.device_id)?;

    match calculator_core::calculate(&req.expression) {
        Ok(result) => {
            repo::insert_success(&state.pool, &req.device_id, &req.expression, result).await?;
            Ok(Json(CalculateResponse::Success { result }))
        }
        Err(calc) => {
            let error_type = match calc.kind {
                CalculationErrorKind::Parse => CalculationErrorType::ParseError,
                CalculationErrorKind::Evaluation => CalculationErrorType::EvaluationError,
            };

            repo::insert_error(
                &state.pool,
                &req.device_id,
                &req.expression,
                error_type,
                &calc.message,
                calc.position,
            )
            .await?;

            Ok(Json(CalculateResponse::Error {
                error_type,
                message: calc.message,
                position: calc.position as u32,
            }))
        }
    }
}

/// GET /api/history?limit=10&device_id=...
pub async fn history(
    State(state): State<AppState>,
    Query(req): Query<HistoryRequest>,
) -> Result<Json<HistoryResponse>, ApiError> {
    validate_device_id(&req.device_id)?;

    let limit = req.limit.min(100) as i64;

    let rows = repo::fetch_history(&state.pool, &req.device_id, limit).await?;
    let entries = rows.into_iter().map(into_history_entry).collect();
    Ok(Json(HistoryResponse(entries)))
}

fn into_history_entry(row: repo::HistoryRow) -> HistoryEntryResponse {
    if let Some(result) = row.result {
        HistoryEntryResponse::Success {
            expression: row.expression,
            result,
            timestamp: row.timestamp,
        }
    } else {
        let error_type = match row.error_type.as_deref() {
            Some("parse_error") => CalculationErrorType::ParseError,
            _ => CalculationErrorType::EvaluationError,
        };
        HistoryEntryResponse::Error {
            expression: row.expression,
            error_type,
            message: row.message.unwrap_or_default(),
            position: row.position.unwrap_or(0) as u32,
            timestamp: row.timestamp,
        }
    }
}
