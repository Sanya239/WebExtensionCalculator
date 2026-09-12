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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_division_by_zero() {
        let expr = Expression::Binary {
            op: BinaryOp::Div,
            left: Box::new(Expression::Number {
                value: 10.0,
                pos: 0,
            }),
            right: Box::new(Expression::Number { value: 0.0, pos: 5 }),
            pos: 3,
        };
        let err = evaluate(&expr).unwrap_err();
        assert_eq!(err.kind, CalculationErrorKind::Evaluation);
        assert_eq!(err.position, 5);
    }

    #[test]
    fn test_unary_minus_evaluation() {
        let expr = Expression::Unary {
            op: UnaryOp::Minus,
            value: Box::new(Expression::Number {
                value: 42.0,
                pos: 1,
            }),
            pos: 0,
        };
        assert!((evaluate(&expr).unwrap() - -42.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_subtraction_associativity() {
        let expr = Expression::Binary {
            op: BinaryOp::Sub,
            left: Box::new(Expression::Binary {
                op: BinaryOp::Sub,
                left: Box::new(Expression::Number {
                    value: 10.0,
                    pos: 0,
                }),
                right: Box::new(Expression::Number { value: 2.0, pos: 5 }),
                pos: 3,
            }),
            right: Box::new(Expression::Number { value: 3.0, pos: 9 }),
            pos: 7,
        };
        assert!((evaluate(&expr).unwrap() - 5.0).abs() < f64::EPSILON);
    }
}
