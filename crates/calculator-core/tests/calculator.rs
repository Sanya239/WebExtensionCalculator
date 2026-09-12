use calculator_core::{CalculationErrorKind, calculate};

#[test]
fn correct_precedence() {
    assert_eq!(calculate("2 + 3 * 4").unwrap(), 14.0);
    assert_eq!(calculate("(2 + 3) * 4").unwrap(), 20.0);
}

#[test]
fn parse_error() {
    let error = calculate("2 + * 3").unwrap_err();
    assert_eq!(error.kind, CalculationErrorKind::Parse);
    assert_eq!(error.position, 4);
}

#[test]
fn division_by_zero_error() {
    let error = calculate("2 / 0 + 3").unwrap_err();
    assert_eq!(error.kind, CalculationErrorKind::Evaluation);
    assert_eq!(error.position, 4);
}
