use std::{error::Error, fmt};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalculationErrorKind {
    Parse,
    Evaluation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CalculationError {
    pub kind: CalculationErrorKind,
    pub message: String,
    pub position: usize,
}

impl fmt::Display for CalculationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} at position {}", self.message, self.position)
    }
}

impl Error for CalculationError {}

impl CalculationError {
    pub fn new(kind: CalculationErrorKind, message: impl Into<String>, position: usize) -> Self {
        Self {
            kind,
            message: message.into(),
            position,
        }
    }
}
