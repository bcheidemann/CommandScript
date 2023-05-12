use lexer::token::{Token, TokenKind, TokenValue};

use crate::{from_token::FromToken, parser_error::ParserError, span::Span};

macro_rules! unwrap_token_value {
    ($variant:ident, $value:expr) => {
        match $value {
            TokenValue::$variant(inner) => inner,
            _ => unreachable!("Unexpected enum variant: {:?}", $value),
        }
    };
}

#[derive(Debug)]
pub struct Program {
    pub ast: Vec<Expression>,
}

#[derive(Debug)]
pub enum Expression {
    Infix(Box<InfixExpression>),
    Prefix(Box<PrefixExpression>),
    Grouping(Box<GroupingExpression>),
    Block(Box<BlockExpression>),
    Literal(Box<LiteralExpression>),
    Identifier(Box<IdentifierExpression>),
    Call(Box<CallExpression>),
    If(Box<IfExpression>),
    Break(Box<BreakExpression>),
    FunctionDeclaration(Box<FunctionDeclarationExpression>),
}

impl Expression {
    pub fn span(&self) -> Span {
        match self {
            Expression::Infix(expression) => *expression.span,
            Expression::Prefix(expression) => *expression.span,
            Expression::Grouping(expression) => *expression.span,
            Expression::Block(expression) => *expression.span,
            Expression::Literal(expression) => *expression.span,
            Expression::Identifier(expression) => *expression.span,
            Expression::Call(expression) => *expression.span,
            Expression::If(expression) => *expression.span,
            Expression::Break(expression) => *expression.span,
            Expression::FunctionDeclaration(expression) => *expression.span,
        }
    }

    pub fn kind_name(&self) -> String {
        match self {
            Expression::Infix(_) => "infix".to_string(),
            Expression::Prefix(_) => "prefix".to_string(),
            Expression::Grouping(_) => "grouping".to_string(),
            Expression::Block(_) => "block".to_string(),
            Expression::Literal(_) => "literal".to_string(),
            Expression::Identifier(_) => "identifier".to_string(),
            Expression::Call(_) => "call".to_string(),
            Expression::If(_) => "if".to_string(),
            Expression::Break(_) => "break".to_string(),
            Expression::FunctionDeclaration(_) => "function declaration".to_string(),
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
    Equals,
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
    ColonEquals,
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
            TokenKind::Equals => Some(Self::Equals),
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
            TokenKind::ColonEquals => Some(Self::ColonEquals),
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
            InfixOperatorKind::Equals => (2, 1),
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
            InfixOperatorKind::ColonEquals => (2, 1),
            InfixOperatorKind::Dot => (7, 8),
            InfixOperatorKind::DotDot => todo!(),
            InfixOperatorKind::Plus | InfixOperatorKind::Minus => (3, 4),
            InfixOperatorKind::Slash | InfixOperatorKind::Star => (5, 6),
            InfixOperatorKind::Caret => todo!(),
            InfixOperatorKind::Percent => todo!(),
        }
    }
}

#[derive(Debug)]
pub enum PostfixOperatorKind {
    BraceSquareOpen,
    BraceRoundOpen,
}

impl PostfixOperatorKind {
    pub fn try_from_token(token: &Token) -> Option<Self> {
        match token.kind {
            TokenKind::BraceSquareOpen => Some(Self::BraceSquareOpen),
            TokenKind::BraceRoundOpen => Some(Self::BraceRoundOpen),
            _ => None,
        }
    }

    pub fn postfix_binding_power(&self) -> (u8, ()) {
        (10, ())
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

impl PrefixOperatorKind {
    pub fn try_from_token(token: &Token) -> Option<Self> {
        match token.kind {
            TokenKind::Bang => Some(Self::Bang),
            TokenKind::Plus => Some(Self::Plus),
            TokenKind::Minus => Some(Self::Minus),
            _ => None,
        }
    }

    pub fn prefix_binding_power(&self) -> ((), u8) {
        ((), 8)
    }
}

#[derive(Debug)]
pub struct CallExpression {
    pub span: Box<Span>,
    pub callee: Box<Expression>,
    pub arguments: Box<Vec<Expression>>,
}

#[derive(Debug)]
pub struct IfExpression {
    pub span: Box<Span>,
    pub conditions: Box<Vec<IfCondition>>,
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
    pub consequence: Box<Expression>,
}

#[derive(Debug)]
pub struct BlockExpression {
    pub span: Box<Span>,
    pub expressions: Box<Vec<Expression>>,
}

#[derive(Debug)]
pub struct BreakExpression {
    pub span: Box<Span>,
    pub expression: Option<Box<Expression>>,
}

#[derive(Debug)]
pub struct IdentifierExpression {
    pub span: Box<Span>,
    pub name: String,
}

impl FromToken for IdentifierExpression {
    fn from_token(token: &Token) -> Result<Self, ParserError> {
        assert!(token.kind == TokenKind::Identifier);
        Ok(
            IdentifierExpression {
                span: Box::new(Span::new(token.start, token.end)),
                name: unwrap_token_value!(String, &token.value).to_string(),
            }
        )
    }
}

#[derive(Debug)]
pub struct GroupingExpression {
    pub span: Box<Span>,
    pub expression: Box<Expression>,
}

#[derive(Debug)]
pub struct FunctionDeclarationExpression {
    pub span: Box<Span>,
    pub parameters: Box<Vec<IdentifierExpression>>,
    pub body: Box<Expression>,
}
