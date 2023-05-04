use ast::{Program, LiteralExpression, Expression, BreakExpression};
use from_token::FromToken;
use lexer::token::{Token, TokenKind};
use parser_error::ParserError;

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
    }
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
    ($self:ident, $kind:ident) => {
        {
            let token = peek_token!($self);

            assert!(
                token.kind == TokenKind::$kind,
                "Expected token of kind {}, found token of kind {}",
                stringify!($kind),
                token.kind
            );

            token
        }
    };
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
        macro_rules! wrap_return_value {
            ($expression_type:expr, $value:expr) => {
                Ok(Some($expression_type(Box::new($value))))
            };
        }

        let token = peek_token!(self);

        match token.kind {
            TokenKind::NewLine => {
                self.advance();
                return Ok(None);
            },
            TokenKind::Identifier => todo!(),
            TokenKind::String => wrap_return_value!(Expression::Literal, self.parse_literal_expression()?),
            TokenKind::Number => wrap_return_value!(Expression::Literal, self.parse_literal_expression()?),
            TokenKind::Boolean => wrap_return_value!(Expression::Literal, self.parse_literal_expression()?),
            TokenKind::Command => todo!(),
            TokenKind::Equals => Err(unexpected_token_error!(token)),
            TokenKind::EqualsEquals => Err(unexpected_token_error!(token)),
            TokenKind::BangEquals => Err(unexpected_token_error!(token)),
            TokenKind::LessThan => Err(unexpected_token_error!(token)),
            TokenKind::LessThanEquals => Err(unexpected_token_error!(token)),
            TokenKind::LessThanLessThan => Err(unexpected_token_error!(token)),
            TokenKind::LessThanLessThanEquals => Err(unexpected_token_error!(token)),
            TokenKind::GreaterThan => Err(unexpected_token_error!(token)),
            TokenKind::GreaterThanEquals => Err(unexpected_token_error!(token)),
            TokenKind::GreaterThanGreaterThan => Err(unexpected_token_error!(token)),
            TokenKind::GreaterThanGreaterThanEquals => Err(unexpected_token_error!(token)),
            TokenKind::SlashEquals => Err(unexpected_token_error!(token)),
            TokenKind::StarEquals => Err(unexpected_token_error!(token)),
            TokenKind::PlusEquals => Err(unexpected_token_error!(token)),
            TokenKind::MinusEquals => Err(unexpected_token_error!(token)),
            TokenKind::PercentEquals => Err(unexpected_token_error!(token)),
            TokenKind::CaretEquals => Err(unexpected_token_error!(token)),
            TokenKind::AmpersandEquals => Err(unexpected_token_error!(token)),
            TokenKind::AmpersandAmpersandEquals => Err(unexpected_token_error!(token)),
            TokenKind::Ampersand => Err(unexpected_token_error!(token)),
            TokenKind::AmpersandAmpersand => Err(unexpected_token_error!(token)),
            TokenKind::PipeEquals => Err(unexpected_token_error!(token)),
            TokenKind::PipePipeEquals => Err(unexpected_token_error!(token)),
            TokenKind::Pipe => Err(unexpected_token_error!(token)),
            TokenKind::PipePipe => Err(unexpected_token_error!(token)),
            TokenKind::Dot => Err(unexpected_token_error!(token)),
            TokenKind::DotDot => todo!(),
            TokenKind::Bang => todo!(),
            TokenKind::Plus => todo!(),
            TokenKind::Minus => todo!(),
            TokenKind::Slash => Err(unexpected_token_error!(token)),
            TokenKind::Star => Err(unexpected_token_error!(token)),
            TokenKind::Caret => Err(unexpected_token_error!(token)),
            TokenKind::Percent => Err(unexpected_token_error!(token)),
            TokenKind::Comma => Err(unexpected_token_error!(token)),
            TokenKind::Comment => todo!(),
            TokenKind::BraceCurlyOpen => todo!(),
            TokenKind::BraceCurlyClose => Err(unexpected_token_error!(token)),
            TokenKind::BraceSquareOpen => todo!(),
            TokenKind::BraceSquareClose => Err(unexpected_token_error!(token)),
            TokenKind::BraceRoundOpen => todo!(),
            TokenKind::BraceRoundClose => Err(unexpected_token_error!(token)),
            TokenKind::If => todo!(),
            TokenKind::Else => Err(unexpected_token_error!(token)),
            TokenKind::For => todo!(),
            TokenKind::While => todo!(),
            TokenKind::Loop => todo!(),
            TokenKind::Break => wrap_return_value!(Expression::Break, self.parse_break_expression()?),
            TokenKind::Continue => todo!(),
            TokenKind::Return => todo!(),
            TokenKind::Whitespace => {
                self.advance();
                return Ok(None);
            },
        }
    }

    fn parse_literal_expression(&mut self) -> Result<LiteralExpression, ParserError> {
        let expression = LiteralExpression::from_token(peek_token!(self));
        self.advance();
        expression
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
}
