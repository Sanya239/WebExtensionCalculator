pub mod error;
pub use error::{CalculationError, CalculationErrorKind};

pub fn calculate(_expression: &str) -> Result<f64, CalculationError> {
    todo!("tokenize string, build AST, evaluate")
}