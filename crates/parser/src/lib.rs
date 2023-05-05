use ast::{BreakExpression, Expression, InfixExpression, LiteralExpression, Program, IdentifierExpression, GroupingExpression};
use from_token::FromToken;
use lexer::token::{Token, TokenKind};
use parser_error::ParserError;

use crate::{ast::InfixOperatorKind, span::Span};

mod from_token;

pub mod ast;
pub mod parser_error;
pub mod span;

macro_rules! unexpected_token_error {
    ($token:ident) => {
        ParserError {
            message: format!("Unexpected token of kind {}", $token.kind),
            position: $token.start,
        }
    };
    ($token:ident, $message:expr) => {
        ParserError {
            message: format!("Unexpected token of kind {}: {}", $token.kind, $message),
            position: $token.start,
        }
    };
}

macro_rules! peek_token {
    ($self:ident) => {
        $self.peek().ok_or(ParserError {
            message: "Unexpected end of file".to_string(),
            position: match $self.tokens.last() {
                Some(token) => token.end,
                None => 0,
            },
        })?
    };
}

macro_rules! peek_assert_token {
    ($self:ident, $kind:ident) => {{
        let token = peek_token!($self);

        assert!(
            token.kind == TokenKind::$kind,
            "Expected token of kind {}, found token of kind {}",
            stringify!($kind),
            token.kind
        );

        token
    }};
}

struct ParserContext {
    pub is_loop: bool,
}

impl Default for ParserContext {
    fn default() -> Self {
        Self { is_loop: false }
    }
}

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    position: usize,
    context: ParserContext,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
            context: ParserContext::default(),
        }
    }

    // === Parser ===

    pub fn parse(&mut self) -> Result<Program, ParserError> {
        let mut program = Program { ast: vec![] };

        while self.position < self.tokens.len() {
            // TODO: If parse error is returned, advance to the next newline token
            //       and collect the error in a vector of errors to be returned
            let expression = self.parse_expression()?;

            // Skip whitespace and newlines
            if let Some(expression) = expression {
                program.ast.push(expression);
            }
        }

        Ok(program)
    }

    fn parse_expression(&mut self) -> Result<Option<Expression>, ParserError> {
        self.pratt_parse_expression(0)
    }

    // Pratt parser for expressions based on https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html
    fn pratt_parse_expression(&mut self, min_bp: u8) -> Result<Option<Expression>, ParserError> {
        macro_rules! wrap_lhs {
            ($expression_type:expr, $value:expr) => {
                $expression_type(Box::new($value))
            };
        }

        self.skip_whitespace();

        let token = peek_token!(self);
        let span = Span::start_from(token.start);

        let mut lhs = match token.kind {
            TokenKind::Whitespace | TokenKind::NewLine => unreachable!("Whitespace and newlines should be skipped"),
            TokenKind::Identifier => wrap_lhs!(Expression::Identifier, self.parse_identifier_expression()?),
            TokenKind::String | TokenKind::Number | TokenKind::Boolean => {
                wrap_lhs!(Expression::Literal, self.parse_literal_expression()?)
            },
            TokenKind::Command => todo!(),
            TokenKind::Equals => return Err(unexpected_token_error!(token)),
            TokenKind::EqualsEquals => return Err(unexpected_token_error!(token)),
            TokenKind::BangEquals => return Err(unexpected_token_error!(token)),
            TokenKind::LessThan => return Err(unexpected_token_error!(token)),
            TokenKind::LessThanEquals => return Err(unexpected_token_error!(token)),
            TokenKind::LessThanLessThan => return Err(unexpected_token_error!(token)),
            TokenKind::LessThanLessThanEquals => return Err(unexpected_token_error!(token)),
            TokenKind::GreaterThan => return Err(unexpected_token_error!(token)),
            TokenKind::GreaterThanEquals => return Err(unexpected_token_error!(token)),
            TokenKind::GreaterThanGreaterThan => return Err(unexpected_token_error!(token)),
            TokenKind::GreaterThanGreaterThanEquals => return Err(unexpected_token_error!(token)),
            TokenKind::SlashEquals => return Err(unexpected_token_error!(token)),
            TokenKind::StarEquals => return Err(unexpected_token_error!(token)),
            TokenKind::PlusEquals => return Err(unexpected_token_error!(token)),
            TokenKind::MinusEquals => return Err(unexpected_token_error!(token)),
            TokenKind::PercentEquals => return Err(unexpected_token_error!(token)),
            TokenKind::CaretEquals => return Err(unexpected_token_error!(token)),
            TokenKind::AmpersandEquals => return Err(unexpected_token_error!(token)),
            TokenKind::AmpersandAmpersandEquals => return Err(unexpected_token_error!(token)),
            TokenKind::Ampersand => return Err(unexpected_token_error!(token)),
            TokenKind::AmpersandAmpersand => return Err(unexpected_token_error!(token)),
            TokenKind::PipeEquals => return Err(unexpected_token_error!(token)),
            TokenKind::PipePipeEquals => return Err(unexpected_token_error!(token)),
            TokenKind::Pipe => return Err(unexpected_token_error!(token)),
            TokenKind::PipePipe => return Err(unexpected_token_error!(token)),
            TokenKind::Dot => return Err(unexpected_token_error!(token)),
            TokenKind::DotDot => todo!(),
            TokenKind::Bang => todo!(),
            TokenKind::Plus => todo!(),
            TokenKind::Minus => todo!(),
            TokenKind::Slash => return Err(unexpected_token_error!(token)),
            TokenKind::Star => return Err(unexpected_token_error!(token)),
            TokenKind::Caret => return Err(unexpected_token_error!(token)),
            TokenKind::Percent => return Err(unexpected_token_error!(token)),
            TokenKind::Comma => return Err(unexpected_token_error!(token)),
            TokenKind::Comment => todo!(),
            TokenKind::BraceCurlyOpen => todo!(),
            TokenKind::BraceCurlyClose => return Err(unexpected_token_error!(token)),
            TokenKind::BraceSquareOpen => todo!(),
            TokenKind::BraceSquareClose => return Err(unexpected_token_error!(token)),
            TokenKind::BraceRoundOpen => wrap_lhs!(Expression::Grouping, self.parse_grouping_expression()?),
            TokenKind::BraceRoundClose => return Err(unexpected_token_error!(token)),
            TokenKind::If => todo!(),
            TokenKind::Else => return Err(unexpected_token_error!(token)),
            TokenKind::For => todo!(),
            TokenKind::While => todo!(),
            TokenKind::Loop => todo!(),
            TokenKind::Break => {
                wrap_lhs!(Expression::Break, self.parse_break_expression()?)
            }
            TokenKind::Continue => todo!(),
            TokenKind::Return => todo!(),
        };


        self.skip_whitespace();

        loop {
            let token = match self.peek() {
                Some(token) => token.clone(),
                None => break,
            };

            let operator = match InfixOperatorKind::try_from_token(&token) {
                Some(operator) => operator,
                None => break,
            };

            let (l_bp, r_bp) = operator.binding_power();

            if l_bp < min_bp {
                break;
            }

            self.advance_and_skip_whitespace();

            let rhs = match self.pratt_parse_expression(r_bp)? {
                Some(rhs) => rhs,
                None => return Err(ParserError {
                    message: "Expected expression".to_string(),
                    position: token.start,
                }),
            };

            lhs = Expression::Infix(Box::new(InfixExpression {
                span: Box::new(span.extend(rhs.span().end)),
                left: Box::new(lhs),
                operator,
                right: Box::new(rhs),
            }));
        }

        Ok(Some(lhs))
    }

    fn parse_grouping_expression(&mut self) -> Result<GroupingExpression, ParserError> {
        let token = peek_assert_token!(self, BraceRoundOpen).clone();
        let span = Span::start_from(token.start);

        self.advance_and_skip_whitespace();

        let expression = self.pratt_parse_expression(0)?.ok_or(ParserError {
            message: "Expected expression".to_string(),
            position: token.end,
        })?;

        self.skip_whitespace();

        let token = peek_token!(self).clone();
        
        if token.kind != TokenKind::BraceRoundClose {
            return Err(unexpected_token_error!(token, "Expected ')'"));
        }

        self.advance();

        Ok(GroupingExpression {
            span: Box::new(span.extend(token.end)),
            expression: Box::new(expression),
        })
    }

    fn parse_literal_expression(&mut self) -> Result<LiteralExpression, ParserError> {
        let expression = LiteralExpression::from_token(peek_token!(self));
        self.advance();
        expression
    }

    fn parse_identifier_expression(&mut self) -> Result<IdentifierExpression, ParserError> {
        let token = peek_assert_token!(self, Identifier);
        let identifier = IdentifierExpression::from_token(token);
        self.advance();
        identifier
    }

    fn parse_break_expression(&mut self) -> Result<BreakExpression, ParserError> {
        let token = peek_assert_token!(self, Break);

        if !self.context.is_loop {
            return Err(ParserError {
                message: "Break expression outside of loop".to_string(),
                position: token.start,
            });
        }

        self.advance();

        todo!("Parse break expression");
    }

    // === Helpers ===

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn skip_whitespace(&mut self) {
        while let Some(token) = self.peek() {
            match token.kind {
                TokenKind::Whitespace | TokenKind::NewLine => self.advance(),
                _ => break,
            }
        }
    }

    fn advance_and_skip_whitespace(&mut self) {
        self.advance();
        self.skip_whitespace();
    }
}
