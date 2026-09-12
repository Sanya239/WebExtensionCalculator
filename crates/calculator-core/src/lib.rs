pub mod error;
pub use error::{CalculationError, CalculationErrorKind};

#[derive(Clone, Copy)]
enum TokenKind {
    Number(f64),
    Plus,
    Minus,
    Asterisk,
    Slash,
    LParen,
    RParen
}

#[derive(Clone, Copy)]
struct Token {
    kind: TokenKind,
    pos: usize
}

fn tokenize(input: &str) -> Result<Vec<Token>, CalculationError> {
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
                tokens.push(Token {kind: TokenKind::Plus, pos });
                i += 1;
            }
            '-' => {
                tokens.push(Token {kind: TokenKind::Minus, pos });
                i += 1;
            }
            '*' => {
                tokens.push(Token {kind: TokenKind::Asterisk, pos });
                i += 1;
            }
            '/' => {
                tokens.push(Token {kind: TokenKind::Slash, pos });
                i += 1;
            }
            '(' => {
                tokens.push(Token {kind: TokenKind::LParen, pos });
                i += 1;
            }
            ')' => {
                tokens.push(Token {kind: TokenKind::RParen, pos });
                i += 1;
            }
            '0'..='9' | '.' => {
                let start_pos = pos;
                let mut number = String::new();
                let mut has_dot = false;
                while i < chars.len() {
                    let (_, char) = chars[i];
                    if char.is_ascii_digit() {
                        number.push(char);
                        i += 1;
                    } else if char == '.' {
                        if has_dot {
                            return Err(CalculationError::new(
                                CalculationErrorKind::Parse,
                                "Invalid float literal: multiple decimal points found.",
                                chars[i].0,
                            ));
                        }
                        has_dot = true;
                        number.push(chars[i].1);
                        i += 1;
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
                        format!("Failed to parse number '{}'.", number),
                        start_pos,
                    )
                })?;
                tokens.push(Token {kind: TokenKind::Number(value), pos: start_pos});
            }
            _ => {
                return Err(CalculationError::new(
                    CalculationErrorKind::Parse,
                    format!("Unexpected character: '{}' found in expression.", char),
                    pos
                ));
            }
        }
    }

    if tokens.is_empty() {
        return Err(CalculationError::new(
            CalculationErrorKind::Parse,
            "Expression is empty.",
            0
        ));
    }

    Ok(tokens)
}

#[derive(PartialEq)]
enum BinaryOp{
    Add,
    Sub,
    Mul,
    Div
}

enum UnaryOp {
    Plus,
    Minus
}

enum Expression {
    Number { value: f64, pos: usize },
    Unary { op: UnaryOp, value: Box<Expression>, pos: usize },
    Binary { op: BinaryOp, left: Box<Expression>, right: Box<Expression>, pos: usize },
}

impl Expression {
    fn pos(&self) -> usize {
        match self {
            Expression::Number { pos, .. } => *pos,
            Expression::Unary { pos, .. } => *pos,
            Expression::Binary { pos, .. } => *pos,
        }
    }
}

fn is_zero(expression: &Expression) -> Option<usize> {
    match expression {
        Expression::Number { value, pos } if *value == 0.0 => Some(*pos),
        Expression::Unary { value, ..} => is_zero(value),
        _ => None,
    }
}

struct Parser<'a> {
    tokens: &'a[Token],
    now: usize,
    total_size: usize,
}

impl<'a> Parser<'a> {
    fn new(tokens: &'a [Token], total_size: usize) -> Parser<'a> {
        Parser {
            tokens,
            now: 0,
            total_size,
        }
    }

    fn look(&self) -> Option<Token> {
        self.tokens.get(self.now).copied()
    }

    fn increment(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.now);
        if token.is_some() {
            self.now += 1;
        }
        token
    }

    fn parse_expression(&mut self) -> Result<Expression, CalculationError> {
        let mut left = self.parse_term()?;
        while let Some(token) = self.look() {
            let (operator, pos) = match token.kind {
                TokenKind::Plus => (BinaryOp::Add, token.pos),
                TokenKind::Minus => (BinaryOp::Sub, token.pos),
                _ => break,
            };
            self.increment();
            let right = self.parse_term()?;
            left = Expression::Binary {
                op: operator,
                left: Box::new(left),
                right: Box::new(right),
                pos,
            };
        }
        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Expression, CalculationError> {
        let mut left = self.parse_factor()?;
        while let Some(token) = self.look() {
            let (operator, pos) = match token.kind {
                TokenKind::Asterisk => (BinaryOp::Mul, token.pos),
                TokenKind::Slash => (BinaryOp::Div, token.pos),
                _ => break,
            };
            self.increment();
            let right = self.parse_factor()?;
            if operator == BinaryOp::Div {
                if let Some(zero_pos) = is_zero(&right) {
                    return Err(CalculationError::new(
                        CalculationErrorKind::Parse,
                        "Division by zero.",
                        zero_pos,
                    ));
                }
            }
            left = Expression::Binary {
                op: operator,
                left: Box::new(left),
                right: Box::new(right),
                pos,
            };
        }
        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<Expression, CalculationError> {
        let token = match self.look() {
            Some(token) => token.clone(),
            None => {
                return Err(CalculationError::new(
                    CalculationErrorKind::Parse,
                    "Unexpected end of expression.",
                    self.total_size,
                ));
            }
        };
        match token.kind {
            TokenKind::Plus => {
                self.increment();
                let expression = self.parse_factor()?;
                Ok(Expression::Unary {
                    op: UnaryOp::Plus,
                    value: Box::from(expression),
                    pos: token.pos,
                })
            }
            TokenKind::Minus => {
                self.increment();
                let expression = self.parse_factor()?;
                Ok(Expression::Unary {
                    op: UnaryOp::Minus,
                    value: Box::from(expression),
                    pos: token.pos,
                })
            }
            _ => self.parse_number(),
        }
    }

    fn parse_number(&mut self) -> Result<Expression, CalculationError> {
        let token = match self.look() {
            Some(token) => token,
            None => {
                return Err(CalculationError::new(
                    CalculationErrorKind::Parse,
                    "Unexpected end of expression.",
                    self.total_size,
                ));
            }
        };
        match token.kind {
            TokenKind::Number(value) => {
                self.increment();
                Ok(Expression::Number { value, pos: token.pos })
            },
            TokenKind::LParen => {
                let lp_pos = token.pos;
                self.increment();
                let expression = self.parse_expression()?;
                match self.look() {
                    Some(Token { kind: TokenKind::RParen, .. }) => {
                        self.increment();
                        Ok(expression)
                    }
                    Some(smth) => Err(CalculationError::new(
                        CalculationErrorKind::Parse,
                        "Expected right parenthesis.",
                        smth.pos
                    )),
                    None => Err(CalculationError::new(
                        CalculationErrorKind::Parse,
                        "Unclosed parenthesis.",
                        lp_pos,
                    )),
                }
            }
            _ => Err(CalculationError::new(
                CalculationErrorKind::Parse,
                format!("Unexpected token at the position: {}", token.pos),
                token.pos,
            )),
        }
    }
}

fn calculator(expression: &Expression) -> Result<f64, CalculationError> {
    match expression {
        Expression::Number { value, pos: _ } => Ok(*value),
        Expression::Unary { op, value: expr, .. } => {
            let value = calculator(expr)?;
            match op {
                UnaryOp::Plus => Ok(value),
                UnaryOp::Minus => Ok(-value),
            }
        }
        Expression::Binary { op, left, right, .. } => {
            let lv = calculator(left)?;
            let rv = calculator(right)?;
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
                },
            }
        }
    }
}

pub fn calculate(expression: &str) -> anyhow::Result<f64> {
    let tokens = tokenize(expression)?;
    let mut parser = Parser::new(&tokens, expression.len());
    let ast = parser.parse_expression()?;
    if let Some(token) = parser.look() {
        return Err(CalculationError::new(
            CalculationErrorKind::Parse,
            "Unexpected token after expression.",
            token.pos,
        ).into());
    }
    let ans = calculator(&ast)?;
    Ok(ans)
}
