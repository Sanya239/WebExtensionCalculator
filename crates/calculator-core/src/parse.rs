use crate::error::{CalculationError, CalculationErrorKind};
use crate::tokenize::{Token, TokenKind};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum UnaryOp {
    Plus,
    Minus,
}

pub(crate) enum Expression {
    Number {
        value: f64,
        pos: usize,
    },
    Unary {
        op: UnaryOp,
        value: Box<Expression>,
        pos: usize,
    },
    Binary {
        op: BinaryOp,
        left: Box<Expression>,
        right: Box<Expression>,
        pos: usize,
    },
}

impl Expression {
    pub(crate) fn pos(&self) -> usize {
        match self {
            Expression::Number { pos, .. }
            | Expression::Unary { pos, .. }
            | Expression::Binary { pos, .. } => *pos,
        }
    }
}

fn is_zero(expression: &Expression) -> Option<usize> {
    match expression {
        Expression::Number { value, pos } if *value == 0.0 => Some(*pos),
        Expression::Unary { value, .. } => is_zero(value),
        _ => None,
    }
}

struct Parser<'a> {
    tokens: &'a [Token],
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

    fn increment(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.now).copied();
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
            if let (BinaryOp::Div, Some(zero_pos)) = (operator, is_zero(&right)) {
                return Err(CalculationError::new(
                    CalculationErrorKind::Parse,
                    "Division by zero.",
                    zero_pos,
                ));
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
        let Some(token) = self.look() else {
            return Err(CalculationError::new(
                CalculationErrorKind::Parse,
                "Unexpected end of expression.",
                self.total_size,
            ));
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
        let Some(token) = self.look() else {
            return Err(CalculationError::new(
                CalculationErrorKind::Parse,
                "Unexpected end of expression.",
                self.total_size,
            ));
        };
        match token.kind {
            TokenKind::Number(value) => {
                self.increment();
                Ok(Expression::Number {
                    value,
                    pos: token.pos,
                })
            }
            TokenKind::LParen => {
                let lp_pos = token.pos;
                self.increment();
                let expression = self.parse_expression()?;
                match self.look() {
                    Some(Token {
                        kind: TokenKind::RParen,
                        ..
                    }) => {
                        self.increment();
                        Ok(expression)
                    }
                    Some(smth) => Err(CalculationError::new(
                        CalculationErrorKind::Parse,
                        "Expected right parenthesis.",
                        smth.pos,
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

pub(crate) fn parse(tokens: &[Token], total_size: usize) -> Result<Expression, CalculationError> {
    let mut parser = Parser::new(tokens, total_size);
    let ast = parser.parse_expression()?;
    if let Some(token) = parser.look() {
        return Err(CalculationError::new(
            CalculationErrorKind::Parse,
            "Unexpected token after expression.",
            token.pos,
        ));
    }
    Ok(ast)
}
