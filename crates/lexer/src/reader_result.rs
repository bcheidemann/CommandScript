use crate::{reader_error, token};

pub enum ReaderResult {
    Err(reader_error::ReaderError),
    None,
    Token(token::Token),
}
