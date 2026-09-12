use crate::error::{CalculationError, CalculationErrorKind};
use crate::parse::{BinaryOp, Expression, UnaryOp};

pub(crate) fn evaluate(expression: &Expression) -> Result<f64, CalculationError> {
    match expression {
        Expression::Number { value, pos: _ } => Ok(*value),
        Expression::Unary {
            op, value: expr, ..
        } => {
            let value = evaluate(expr)?;
            match op {
                UnaryOp::Plus => Ok(value),
                UnaryOp::Minus => Ok(-value),
            }
        }
        Expression::Binary {
            op, left, right, ..
        } => {
            let lv = evaluate(left)?;
            let rv = evaluate(right)?;
            match op {
                BinaryOp::Add => Ok(lv + rv),
                BinaryOp::Sub => Ok(lv - rv),
                BinaryOp::Mul => Ok(lv * rv),
                BinaryOp::Div => {
                    if rv == 0.0 {
                        return Err(CalculationError::new(
                            CalculationErrorKind::Evaluation,
                            "Division by zero.",
                            right.pos(),
                        ));
                    }
                    Ok(lv / rv)
                }
            }
        }
    }
}
