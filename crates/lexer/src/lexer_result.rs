use crate::{reader_error, token};

#[derive(Debug)]
pub struct LexerResult {
    pub tokens: Vec<token::Token>,
    pub errors: Vec<reader_error::ReaderError>,
}
