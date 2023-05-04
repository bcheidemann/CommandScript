use lexer::token::{Token, TokenKind, TokenValue};

use crate::{from_token::FromToken, parser_error::ParserError};

#[derive(Debug)]
pub struct Program {
    pub ast: Vec<Expression>,
}

#[derive(Debug)]
pub enum Expression {
    Literal(Box<LiteralExpression>),
    Infix(Box<InfixExpression>),
    Prefix(Box<PrefixExpression>),
    If(Box<IfExpression>),
    Block(Box<BlockExpression>),
    Break(Box<BreakExpression>),
}

#[derive(Debug)]
pub enum LiteralExpression {
    String(String),
    Number(f64),
    Boolean(bool),
}

impl FromToken for LiteralExpression {
    fn from_token(token: &Token) -> Result<Self, ParserError> {
        match &token.kind {
            TokenKind::String => {
                if let TokenValue::String(value) = &token.value {
                    Ok(Self::String(value.to_string()))
                } else {
                    unreachable!("Token of kind String must have a value of type String");
                }
            }
            TokenKind::Number => {
                if let TokenValue::Number(value) = token.value {
                    Ok(Self::Number(value))
                } else {
                    unreachable!("Token of kind Number must have a value of type Number");
                }
            },
            TokenKind::Boolean => {
                if let TokenValue::Boolean(value) = token.value {
                    Ok(Self::Boolean(value))
                } else {
                    unreachable!("Token of kind Boolean must have a value of type Boolean");
                }
            },
            kind => Err(ParserError {
                message: format!("Token of kind {kind} is not a valid literal expression"),
                position: token.start,
            }),
        }
    }
}

#[derive(Debug)]
pub struct InfixExpression {
    pub left: Box<Expression>,
    pub operator: InfixOperatorKind,
    pub right: Box<Expression>,
}

#[derive(Debug)]
pub enum InfixOperatorKind {
    EqualsEquals,
    BangEquals,
    LessThan,
    LessThanEquals,
    LessThanLessThan,
    GreaterThan,
    GreaterThanEquals,
    GreaterThanGreaterThan,
    Ampersand,
    AmpersandAmpersand,
    Pipe,
    PipePipe,
    Dot,
    DotDot,
    Plus,
    Minus,
    Slash,
    Star,
    Caret,
    Percent,
}

#[derive(Debug)]
pub struct PrefixExpression {
    pub operator: PrefixOperatorKind,
    pub right: Box<Expression>,
}

#[derive(Debug)]
pub enum PrefixOperatorKind {
    Bang,
    Plus,
    Minus,
}

#[derive(Debug)]
pub struct IfExpression {
    pub conditions: Vec<IfCondition>,
    pub default: Option<Box<Expression>>,
}

#[derive(Debug)]
pub struct IfCondition {
    pub condition: Box<Expression>,
    pub consequence: Box<Expression>,
}

#[derive(Debug)]
pub struct BlockExpression {
    pub expressions: Vec<Expression>,
}

#[derive(Debug)]
pub struct BreakExpression {
    pub expression: Option<Box<Expression>>,
}
