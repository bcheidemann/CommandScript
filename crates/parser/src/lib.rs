use ast::{BreakExpression, Expression, InfixExpression, LiteralExpression, Program, IdentifierExpression, GroupingExpression, PrefixExpression, PrefixOperatorKind, BlockExpression, IfExpression, CallExpression};
use from_token::FromToken;
use lexer::token::{Token, TokenKind};
use parser_error::ParserError;

use crate::{ast::{InfixOperatorKind, IfCondition, IfDefault, PostfixOperatorKind}, span::Span};

mod from_token;

pub mod ast;
pub mod parser_error;
pub mod span;

macro_rules! unexpected_token_error {
    ($token:expr) => {
        ParserError {
            message: format!("Unexpected token of kind {}", $token.kind),
            position: $token.start,
        }
    };
    ($token:expr, $message:expr) => {
        ParserError {
            message: format!("Unexpected token of kind {}: {}", $token.kind, $message),
            position: $token.start,
        }
    };
}

macro_rules! expected_expression_error {
    ($token:expr) => {
        ParserError {
            message: "Expected expression".to_string(),
            position: $token.end,
        }
    };
}

macro_rules! peek_token {
    ($self:expr) => {
        $self.peek().ok_or(ParserError {
            message: "Unexpected end of file".to_string(),
            position: match $self.tokens.last() {
                Some(token) => token.end,
                None => 0,
            },
        })?
    };
}

macro_rules! assert_token {
    ($self:ident, $kind:ident) => {
        let token = peek_token!($self);

        assert!(
            token.kind == TokenKind::$kind,
            "Expected token of kind {}, found token of kind {}",
            stringify!($kind),
            token.kind
        );
    }
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

macro_rules! peek_assert_matching_kind {
    ($self:ident, $kind:pat) => {{
        let token = peek_token!($self);

        assert!(
            matches!(token.kind, $kind),
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

        let token = match self.peek() {
            Some(token) => token,
            None => return Ok(None),
        };
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
            TokenKind::Bang | TokenKind::Plus | TokenKind::Minus => {
                wrap_lhs!(Expression::Prefix, self.parse_prefix_expression()?)
            },
            TokenKind::Slash => return Err(unexpected_token_error!(token)),
            TokenKind::Star => return Err(unexpected_token_error!(token)),
            TokenKind::Caret => return Err(unexpected_token_error!(token)),
            TokenKind::Percent => return Err(unexpected_token_error!(token)),
            TokenKind::Comma => return Err(unexpected_token_error!(token)),
            TokenKind::Comment => todo!(),
            TokenKind::BraceCurlyOpen => wrap_lhs!(Expression::Block, self.parse_block_expression()?),
            TokenKind::BraceCurlyClose => return Err(unexpected_token_error!(token)),
            TokenKind::BraceSquareOpen => todo!(),
            TokenKind::BraceSquareClose => return Err(unexpected_token_error!(token)),
            TokenKind::BraceRoundOpen => wrap_lhs!(Expression::Grouping, self.parse_grouping_expression()?),
            TokenKind::BraceRoundClose => return Err(unexpected_token_error!(token)),
            TokenKind::If => wrap_lhs!(Expression::If, self.parse_if_expression()?),
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

        loop {
            self.skip_whitespace();

            let token = match self.peek() {
                Some(token) => token.clone(),
                None => break,
            };

            if let Some(operator) = PostfixOperatorKind::try_from_token(&token) {
                let (l_bp, ()) = operator.postfix_binding_power();

                if l_bp < min_bp {
                    break;
                }

                lhs = match operator {
                    PostfixOperatorKind::BraceSquareOpen => todo!(),
                    PostfixOperatorKind::BraceRoundOpen => Expression::Call(Box::new(self.parse_call_expression(lhs)?)),
                }
            }

            else if let Some(operator) = InfixOperatorKind::try_from_token(&token) {
                let (l_bp, r_bp) = operator.binding_power();

                if l_bp < min_bp {
                    break;
                }

                self.advance_and_skip_whitespace();

                let rhs = match self.pratt_parse_expression(r_bp)? {
                    Some(rhs) => rhs,
                    None => return Err(expected_expression_error!(token)),
                };

                lhs = Expression::Infix(Box::new(InfixExpression {
                    span: Box::new(span.extend(rhs.span().end)),
                    left: Box::new(lhs),
                    operator,
                    right: Box::new(rhs),
                }));
            }

            else { break };
        }

        Ok(Some(lhs))
    }

    fn parse_call_expression(&mut self, callee: Expression) -> Result<CallExpression, ParserError> {
        assert_token!(self, BraceRoundOpen);
        let span = callee.span();

        self.advance_and_skip_whitespace();

        let mut arguments = vec![];

        loop {
            let token = peek_token!(self).clone();

            if token.kind == TokenKind::BraceRoundClose {
                self.advance();

                return Ok(CallExpression {
                    span: Box::new(span.extend(token.end)),
                    callee: Box::new(callee),
                    arguments: Box::new(arguments),
                });
            }

            // TODO: If parse error is returned, advance to the next newline token
            //       and collect the error in a vector of errors to be returned
            let expression = self.parse_expression()?;

            // Skip whitespace and newlines
            if let Some(expression) = expression {
                arguments.push(expression);
            }
        }
    }

    fn parse_block_expression(&mut self) -> Result<BlockExpression, ParserError> {
        let token = peek_assert_token!(self, BraceCurlyOpen).clone();
        let span = Span::start_from(token.start);

        self.advance_and_skip_whitespace();

        let mut expressions = vec![];

        loop {
            let token = peek_token!(self).clone();

            if token.kind == TokenKind::BraceCurlyClose {
                self.advance();

                return Ok(BlockExpression {
                    span: Box::new(span.extend(token.end)),
                    expressions: Box::new(expressions),
                });
            }

            // TODO: If parse error is returned, advance to the next newline token
            //       and collect the error in a vector of errors to be returned
            let expression = self.parse_expression()?;

            // Skip whitespace and newlines
            if let Some(expression) = expression {
                expressions.push(expression);
            }
        }
    }

    fn parse_prefix_expression(&mut self) -> Result<PrefixExpression, ParserError> {
        let token = peek_assert_matching_kind!(
            self,
            TokenKind::Bang | TokenKind::Minus | TokenKind::Plus
        ).clone();
        let span = Span::start_from(token.start);
        let operator = match PrefixOperatorKind::try_from_token(&token) {
            Some(operator) => operator,
            None => unreachable!(),
        };
        let ((), r_bp) = operator.prefix_binding_power();

        self.advance_and_skip_whitespace();

        let expression = self.pratt_parse_expression(r_bp)?.ok_or(expected_expression_error!(token))?;

        Ok(PrefixExpression {
            span: Box::new(span.extend(expression.span().end)),
            operator,
            right: Box::new(expression),
        })
    }

    fn parse_grouping_expression(&mut self) -> Result<GroupingExpression, ParserError> {
        let token = peek_assert_token!(self, BraceRoundOpen).clone();
        let span = Span::start_from(token.start);

        self.advance_and_skip_whitespace();

        let expression = self.parse_expression()?.ok_or(expected_expression_error!(token))?;

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

    fn parse_if_expression(&mut self) -> Result<IfExpression, ParserError> {
        let token = peek_assert_token!(self, If).clone();
        let mut outer_span = Span::start_from(token.start);

        self.advance_and_skip_whitespace();

        let mut conditions = vec![{
            let condition = self.parse_expression()?.ok_or(expected_expression_error!(token))?;
            let token = peek_token!(self).clone();
            let consequence = self.parse_expression()?.ok_or(expected_expression_error!(token))?;
            let span = outer_span.clone().extend(consequence.span().end);

            outer_span = outer_span.extend(span.end);

            IfCondition {
                span: Box::new(span),
                condition: Box::new(condition),
                consequence: Box::new(consequence),
            }
        }];
        let mut default = None;

        self.skip_whitespace();

        while let Some(token) = self.peek() {
            if token.kind != TokenKind::Else {
                break;
            }

            let span = Span::start_from(token.start);

            self.advance_and_skip_whitespace();

            let token = peek_token!(self).clone();

            if token.kind == TokenKind::If {
                self.advance_and_skip_whitespace();

                conditions.push({
                    let condition = self.parse_expression()?.ok_or(expected_expression_error!(token))?;
                    let token = peek_token!(self).clone();
                    let consequence = self.parse_expression()?.ok_or(expected_expression_error!(token))?;
                    let span = span.extend(consequence.span().end);

                    outer_span = outer_span.extend(span.end);

                    self.skip_whitespace();

                    IfCondition {
                        span: Box::new(span),
                        condition: Box::new(condition),
                        consequence: Box::new(consequence),
                    }
                });
            }
            else {
                let consequence = self.parse_expression()?.ok_or(expected_expression_error!(token))?;
                let span = span.extend(consequence.span().end);

                outer_span = outer_span.extend(span.end);

                default = Some(Box::new(IfDefault {
                    span: Box::new(span),
                    consequence: Box::new(consequence),
                }));

                break;
            }
        }

        Ok(IfExpression {
            span: Box::new(outer_span),
            conditions: Box::new(conditions),
            default,
        })
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
