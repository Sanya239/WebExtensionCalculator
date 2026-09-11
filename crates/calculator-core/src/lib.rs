pub mod error;
pub use error::{CalculationError, CalculationErrorKind};

pub fn calculate(_expression: &str) -> anyhow::Result<f64> {
    todo!("tokenize string, build AST, evaluate")
}
