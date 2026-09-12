use crate::error::CalculationError;

pub mod error;
mod evaluate;
mod parse;
mod tokenize;

pub fn calculate(expression: &str) -> Result<f64, CalculationError> {
    let tokens = tokenize::tokenize(expression)?;
    let ast = parse::parse(&tokens, expression.len())?;
    let ans = evaluate::evaluate(&ast)?;
    Ok(ans)
}
