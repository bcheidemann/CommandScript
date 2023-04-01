pub mod lexer;
pub mod lexer_result;
pub mod lexer_state;
pub mod reader;
pub mod reader_error;
pub mod reader_result;
pub mod reader_state;
pub mod token;

use reader_error::ReaderError;
use reader_result::ReaderResult;
use reader_state::ReaderState;
use token::{Token, TokenKind, TokenValue};
use unicode_id_start::{is_id_continue, is_id_start};

struct IdentifierReader;

impl reader::Reader for IdentifierReader {
    fn name(&self) -> String {
        "IdentifierReader".to_string()
    }

    fn read(&self, state: &mut ReaderState) -> ReaderResult {
        let mut value = String::new();

        // Check if the first character has the ID_Start property according to the
        // Unicode Standard Annex #31: Unicode Identifier and Pattern Syntax
        // See https://www.unicode.org/reports/tr31/
        if matches!(state.peek(), Some(char) if is_id_start(*char)) {
            value += &state.read().unwrap().to_string();
        } else {
            return ReaderResult::None;
        }

        while matches!(state.peek(), Some(char) if is_id_continue(*char)) {
            value += &state.read().unwrap().to_string();
        }

        return ReaderResult::Token(Token {
            kind: TokenKind::Identifier,
            start: state.get_start(),
            end: state.get_position(),
            value: TokenValue::String(value),
        });
    }
}

struct NewLineReader;

impl reader::Reader for NewLineReader {
    fn name(&self) -> String {
        "NewLineReader".to_string()
    }

    fn read(&self, state: &mut ReaderState) -> ReaderResult {
        if state.peek() == Some(&'\n') {
            state.read();

            return ReaderResult::Token(Token {
                kind: TokenKind::NewLine,
                start: state.get_start(),
                end: state.get_position(),
                value: TokenValue::None,
            });
        }

        return ReaderResult::None;
    }
}

struct WhitespaceReader;

impl reader::Reader for WhitespaceReader {
    fn name(&self) -> String {
        "WhitespaceReader".to_string()
    }

    fn read(&self, state: &mut ReaderState) -> ReaderResult {
        while matches!(state.peek(), Some(char) if char.is_whitespace()) {
            state.read();
        }

        if state.did_advance() {
            return ReaderResult::Token(Token {
                kind: TokenKind::Whitespace,
                start: state.get_start(),
                end: state.get_position(),
                value: TokenValue::None,
            });
        }

        return ReaderResult::None;
    }
}

struct UnexpectedCharacterReader;

impl reader::Reader for UnexpectedCharacterReader {
    fn name(&self) -> String {
        "UnexpectedCharacterReader".to_string()
    }

    fn read(&self, state: &mut ReaderState) -> ReaderResult {
        let char = state.read().unwrap();

        return ReaderResult::Err(ReaderError {
            message: format!("Unexpected character '{}'", char),
            position: state.get_start(),
        });
    }
}

pub fn test() {
    let lexer = lexer::Lexer::new();
    let mut lexer = lexer
        .add_reader(IdentifierReader)
        .add_reader(NewLineReader)
        .add_reader(WhitespaceReader)
        .add_reader(UnexpectedCharacterReader);

    let string =
        "\
        a = 40
        b = 2
        c = a + b + c
        ";

    let result = lexer.lex(&string);

    println!("{result:#?}");
}
