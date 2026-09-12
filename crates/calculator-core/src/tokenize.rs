use crate::error::{CalculationError, CalculationErrorKind};

#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) enum TokenKind {
    Number(f64),
    Plus,
    Minus,
    Asterisk,
    Slash,
    LParen,
    RParen,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) struct Token {
    pub(crate) kind: TokenKind,
    pub(crate) pos: usize,
}

fn tokenize_number(
    chars: &[(usize, char)],
    start_idx: &mut usize,
) -> Result<Token, CalculationError> {
    let start_pos = chars[*start_idx].0;
    let mut number = String::new();
    let mut has_dot = false;
    while *start_idx < chars.len() {
        let (_, char) = chars[*start_idx];
        if char.is_ascii_digit() {
            number.push(char);
            *start_idx += 1;
        } else if char == '.' {
            if has_dot {
                return Err(CalculationError::new(
                    CalculationErrorKind::Parse,
                    "Invalid float literal: multiple decimal points found.",
                    chars[*start_idx].0,
                ));
            }
            has_dot = true;
            number.push(chars[*start_idx].1);
            *start_idx += 1;
        } else {
            break;
        }
    }
    if number == "." {
        return Err(CalculationError::new(
            CalculationErrorKind::Parse,
            "Expected digits before or after the decimal point.",
            start_pos,
        ));
    }
    let value = number.parse::<f64>().map_err(|_| {
        CalculationError::new(
            CalculationErrorKind::Parse,
            format!("Failed to parse number '{number}'."),
            start_pos,
        )
    })?;
    Ok(Token {
        kind: TokenKind::Number(value),
        pos: start_pos,
    })
}

pub(crate) fn tokenize(input: &str) -> Result<Vec<Token>, CalculationError> {
    let mut tokens = Vec::new();
    let chars: Vec<(usize, char)> = input.char_indices().collect();
    let mut i = 0;
    while i < chars.len() {
        let (pos, char) = chars[i];
        match char {
            ' ' | '\n' | '\t' | '\r' => {
                i += 1;
            }
            '+' => {
                tokens.push(Token {
                    kind: TokenKind::Plus,
                    pos,
                });
                i += 1;
            }
            '-' => {
                tokens.push(Token {
                    kind: TokenKind::Minus,
                    pos,
                });
                i += 1;
            }
            '*' => {
                tokens.push(Token {
                    kind: TokenKind::Asterisk,
                    pos,
                });
                i += 1;
            }
            '/' => {
                tokens.push(Token {
                    kind: TokenKind::Slash,
                    pos,
                });
                i += 1;
            }
            '(' => {
                tokens.push(Token {
                    kind: TokenKind::LParen,
                    pos,
                });
                i += 1;
            }
            ')' => {
                tokens.push(Token {
                    kind: TokenKind::RParen,
                    pos,
                });
                i += 1;
            }
            '0'..='9' | '.' => {
                tokens.push(tokenize_number(&chars, &mut i)?);
            }
            _ => {
                return Err(CalculationError::new(
                    CalculationErrorKind::Parse,
                    format!("Unexpected character: '{char}' found in expression."),
                    pos,
                ));
            }
        }
    }

    if tokens.is_empty() {
        return Err(CalculationError::new(
            CalculationErrorKind::Parse,
            "Expression is empty.",
            0,
        ));
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_tokens_and_positions() {
        let tokens = tokenize("2 + 2.75").unwrap();
        assert_eq!(tokens.len(), 3);
        assert_eq!(
            tokens[0],
            Token {
                kind: TokenKind::Number(2.0),
                pos: 0
            }
        );
        assert_eq!(
            tokens[1],
            Token {
                kind: TokenKind::Plus,
                pos: 2
            }
        );
        assert_eq!(
            tokens[2],
            Token {
                kind: TokenKind::Number(2.75),
                pos: 4
            }
        );
    }

    #[test]
    fn test_multiple_decimal_points() {
        let error = tokenize("1.5.2").unwrap_err();
        assert_eq!(error.kind, CalculationErrorKind::Parse);
        assert_eq!(error.position, 3);
    }

    #[test]
    fn test_alone_decimal_point() {
        let error = tokenize("6 + .").unwrap_err();
        assert_eq!(error.kind, CalculationErrorKind::Parse);
        assert_eq!(error.position, 4);
    }

    #[test]
    fn test_unexpected_char() {
        let error = tokenize("5 + 2 # 7").unwrap_err();
        assert_eq!(error.kind, CalculationErrorKind::Parse);
        assert_eq!(error.position, 6);
    }
}
