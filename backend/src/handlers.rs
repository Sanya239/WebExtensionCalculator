use axum::{
    extract::{Query, State},
    Json,
};
use calculator_backend::{
    CalculateRequest, CalculateResponse, CalculationErrorType, HistoryEntryResponse,
    HistoryRequest, HistoryResponse,
};
use calculator_core::{calculate as evaluate, CalculationError, CalculationErrorKind};

use crate::{ApiError, AppState};

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

    match evaluate(&req.expression) {
        Ok(result) => {
            save_success(&state.pool, &req.device_id, &req.expression, result).await?;
            Ok(Json(CalculateResponse::Success { result }))
        }
        Err(err) => {
            let calc = err.downcast_ref::<CalculationError>().cloned().unwrap_or_else(|| {
                CalculationError::new(
                    CalculationErrorKind::Evaluation,
                    err.to_string(),
                    0,
                )
            });

            let error_type = match calc.kind {
                CalculationErrorKind::Parse => CalculationErrorType::ParseError,
                CalculationErrorKind::Evaluation => CalculationErrorType::EvaluationError,
            };

            save_error(
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
                message: calc.message.clone(),
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

    let rows = sqlx::query_as::<_, HistoryRow>(
        r#"
        SELECT expression, result, error_type, message, position, timestamp
        FROM calculations
        WHERE device_id = $1
        ORDER BY timestamp DESC, id DESC
        LIMIT $2
        "#,
    )
    .bind(&req.device_id)
    .bind(limit)
    .fetch_all(&state.pool)
    .await?;

    let entries = rows.into_iter().map(HistoryRow::into_response).collect();
    Ok(Json(HistoryResponse(entries)))
}

/// sql-запрос в rust-структуру
#[derive(sqlx::FromRow)]
struct HistoryRow {
    expression: String,
    result: Option<f64>,
    error_type: Option<String>,
    message: Option<String>,
    position: Option<i32>,
    timestamp: chrono::DateTime<chrono::Utc>,
}

impl HistoryRow {
    fn into_response(self) -> HistoryEntryResponse {
        if let Some(result) = self.result {
            HistoryEntryResponse::Success {
                expression: self.expression,
                result,
                timestamp: self.timestamp,
            }
        } else {
            let error_type = match self.error_type.as_deref() {
                Some("parse_error") => CalculationErrorType::ParseError,
                _ => CalculationErrorType::EvaluationError,
            };
            HistoryEntryResponse::Error {
                expression: self.expression,
                error_type,
                message: self.message.unwrap_or_default(),
                position: self.position.unwrap_or(0) as u32,
                timestamp: self.timestamp,
            }
        }
    }
}

async fn save_success(
    pool: &sqlx::PgPool,
    device_id: &str,
    expression: &str,
    result: f64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO calculations (device_id, expression, result) VALUES ($1, $2, $3)",
    )
    .bind(device_id)
    .bind(expression)
    .bind(result)
    .execute(pool)
    .await?;
    Ok(())
}

async fn save_error(
    pool: &sqlx::PgPool,
    device_id: &str,
    expression: &str,
    error_type: CalculationErrorType,
    message: &str,
    position: usize,
) -> Result<(), sqlx::Error> {
    let error_type_str = match error_type {
        CalculationErrorType::ParseError => "parse_error",
        CalculationErrorType::EvaluationError => "evaluation_error",
    };
    sqlx::query(
        r#"INSERT INTO calculations (device_id, expression, error_type, message, position)
           VALUES ($1, $2, $3, $4, $5)"#,
    )
    .bind(device_id)
    .bind(expression)
    .bind(error_type_str)
    .bind(message)
    .bind(position as i32)
    .execute(pool)
    .await?;
    Ok(())
}