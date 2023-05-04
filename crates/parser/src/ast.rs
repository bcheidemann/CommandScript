use lexer::token::{Token, TokenKind, TokenValue};

use crate::{from_token::FromToken, parser_error::ParserError, span::Span};

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

impl Expression {
    pub fn span(&self) -> &Span {
        match self {
            Self::Literal(expression) => &expression.span,
            Self::Infix(expression) => &expression.span,
            Self::Prefix(expression) => &expression.span,
            Self::If(expression) => &expression.span,
            Self::Block(expression) => &expression.span,
            Self::Break(expression) => &expression.span,
        }
    }
}

#[derive(Debug)]
pub struct LiteralExpression {
    pub span: Box<Span>,
    pub value: Box<LiteralExpressionValue>,
}

impl FromToken for LiteralExpression {
    fn from_token(token: &Token) -> Result<Self, ParserError> {
        Ok(Self {
            span: Box::new(Span::new(token.start, token.end)),
            value: Box::new(LiteralExpressionValue::from_token(token)?),
        })
    }
}

#[derive(Debug)]
pub enum LiteralExpressionValue {
    String(String),
    Number(f64),
    Boolean(bool),
}

impl FromToken for LiteralExpressionValue {
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
    pub span: Box<Span>,
    pub left: Box<Expression>,
    pub operator: InfixOperatorKind,
    pub right: Box<Expression>,
}

#[derive(Debug, Clone, Copy)]
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

impl InfixOperatorKind {
    pub fn try_from_token(token: &Token) -> Option<Self> {
        match token.kind {
            TokenKind::EqualsEquals => Some(Self::EqualsEquals),
            TokenKind::BangEquals => Some(Self::BangEquals),
            TokenKind::LessThan => Some(Self::LessThan),
            TokenKind::LessThanEquals => Some(Self::LessThanEquals),
            TokenKind::LessThanLessThan => Some(Self::LessThanLessThan),
            TokenKind::GreaterThan => Some(Self::GreaterThan),
            TokenKind::GreaterThanEquals => Some(Self::GreaterThanEquals),
            TokenKind::GreaterThanGreaterThan => Some(Self::GreaterThanGreaterThan),
            TokenKind::Ampersand => Some(Self::Ampersand),
            TokenKind::AmpersandAmpersand => Some(Self::AmpersandAmpersand),
            TokenKind::Pipe => Some(Self::Pipe),
            TokenKind::PipePipe => Some(Self::PipePipe),
            TokenKind::Dot => Some(Self::Dot),
            TokenKind::DotDot => Some(Self::DotDot),
            TokenKind::Plus => Some(Self::Plus),
            TokenKind::Minus => Some(Self::Minus),
            TokenKind::Slash => Some(Self::Slash),
            TokenKind::Star => Some(Self::Star),
            TokenKind::Caret => Some(Self::Caret),
            TokenKind::Percent => Some(Self::Percent),
            _ => None,
        }
    }

    pub fn binding_power(&self) -> (u8, u8) {
        match self {
            InfixOperatorKind::EqualsEquals => todo!(),
            InfixOperatorKind::BangEquals => todo!(),
            InfixOperatorKind::LessThan => todo!(),
            InfixOperatorKind::LessThanEquals => todo!(),
            InfixOperatorKind::LessThanLessThan => todo!(),
            InfixOperatorKind::GreaterThan => todo!(),
            InfixOperatorKind::GreaterThanEquals => todo!(),
            InfixOperatorKind::GreaterThanGreaterThan => todo!(),
            InfixOperatorKind::Ampersand => todo!(),
            InfixOperatorKind::AmpersandAmpersand => todo!(),
            InfixOperatorKind::Pipe => todo!(),
            InfixOperatorKind::PipePipe => todo!(),
            InfixOperatorKind::Dot => todo!(),
            InfixOperatorKind::DotDot => todo!(),
            InfixOperatorKind::Plus | InfixOperatorKind::Minus => (1, 2),
            InfixOperatorKind::Slash | InfixOperatorKind::Star => (3, 4),
            InfixOperatorKind::Caret => todo!(),
            InfixOperatorKind::Percent => todo!(),
        }
    }
}

#[derive(Debug)]
pub struct PrefixExpression {
    pub span: Box<Span>,
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
    pub span: Box<Span>,
    pub conditions: Vec<IfCondition>,
    pub default: Option<Box<IfDefault>>,
}

#[derive(Debug)]
pub struct IfCondition {
    pub span: Box<Span>,
    pub condition: Box<Expression>,
    pub consequence: Box<Expression>,
}

#[derive(Debug)]
pub struct IfDefault {
    pub span: Box<Span>,
    pub default: Box<Expression>,
}

#[derive(Debug)]
pub struct BlockExpression {
    pub span: Box<Span>,
    pub expressions: Vec<Expression>,
}

#[derive(Debug)]
pub struct BreakExpression {
    pub span: Box<Span>,
    pub expression: Option<Box<Expression>>,
}
