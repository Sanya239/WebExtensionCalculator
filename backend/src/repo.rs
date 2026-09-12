use calculator_backend::CalculationErrorType;
use chrono::{DateTime, Utc};
use sqlx::PgPool;

#[derive(Debug, sqlx::FromRow)]
pub struct HistoryRow {
    pub expression: String,
    pub result: Option<f64>,
    pub error_type: Option<String>,
    pub message: Option<String>,
    pub position: Option<i32>,
    pub timestamp: DateTime<Utc>,
}

pub async fn insert_success(
    pool: &PgPool,
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

pub async fn insert_error(
    pool: &PgPool,
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

pub async fn fetch_history(
    pool: &PgPool,
    device_id: &str,
    limit: i64,
) -> Result<Vec<HistoryRow>, sqlx::Error> {
    sqlx::query_as::<_, HistoryRow>(
        r#"
        SELECT expression, result, error_type, message, position, timestamp
        FROM calculations
        WHERE device_id = $1
        ORDER BY timestamp DESC, id DESC
        LIMIT $2
        "#,
    )
    .bind(device_id)
    .bind(limit)
    .fetch_all(pool)
    .await
}