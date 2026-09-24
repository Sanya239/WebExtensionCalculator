pub mod error;
mod evaluate;
mod parse;
mod tokenize;

pub use error::{CalculationError, CalculationErrorKind};

/// # Errors
///
/// Will return a [`CalculationError`] if tokenization, syntax parsing (AST building),
/// or mathematical evaluation fails (e.g., syntax errors, unclosed parentheses, division by zero).
pub fn calculate(expression: &str) -> Result<f64, CalculationError> {
    let tokens = tokenize::tokenize(expression)?;
    let ast = parse::parse(&tokens, expression.len())?;
    let ans = evaluate::evaluate(&ast)?;
    Ok(ans)
}
