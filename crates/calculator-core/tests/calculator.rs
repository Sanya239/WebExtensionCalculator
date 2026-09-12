use calculator_core::calculate;
use calculator_core::error::CalculationErrorKind;

#[test]
fn correct_precedence() {
    assert!((calculate("2 + 3 * 4").unwrap() - 14.0).abs() < f64::EPSILON);
    assert!((calculate("(2 + 3) * 4").unwrap() - 20.0).abs() < f64::EPSILON);
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
