use lexer::token::Token;

use crate::parser_error::ParserError;

pub trait FromToken where Self: Sized {
  fn from_token(token: &Token) -> Result<Self, ParserError>;
}
