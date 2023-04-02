pub mod lexer;
pub mod lexer_result;
pub mod lexer_state;
pub mod reader;
pub mod reader_error;
pub mod reader_result;
pub mod reader_state;
pub mod token;

use reader::Reader;
use reader_error::ReaderError;
use reader_result::ReaderResult;
use reader_state::ReaderState;
use token::{Token, TokenKind, TokenValue};
use unicode_id_start::{is_id_continue, is_id_start};

struct IdentifierReader;

impl Reader for IdentifierReader {
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

struct NumberReader;

impl Reader for NumberReader {
    fn name(&self) -> String {
        "NumberReader".to_string()
    }

    fn read(&self, state: &mut ReaderState) -> ReaderResult {
        let mut value = String::new();

        // Read all numeric characters
        while matches!(state.peek(), Some(char) if char.is_numeric()) {
            value += &state.read().unwrap().to_string();
        }

        // There must be at least one numeric character before a dot
        if value.is_empty() {
            return ReaderResult::None;
        }

        // Check if the next character is a dot
        if matches!(state.peek(), Some('.')) {
            value += &state.read().unwrap().to_string();

            // Read all numeric characters after the dot
            while matches!(state.peek(), Some(char) if char.is_numeric()) {
                value += &state.read().unwrap().to_string();
            }
        }

        return ReaderResult::Token(Token {
            kind: TokenKind::Number,
            start: state.get_start(),
            end: state.get_position(),
            value: TokenValue::Number(value.parse().unwrap()),
        });
    }
}

struct OperatorReader;

impl OperatorReader {
    fn get_readers_result(&self, kind: TokenKind, state: &mut ReaderState) -> ReaderResult {
        state.read();

        return ReaderResult::Token(Token {
            kind,
            start: state.get_start(),
            end: state.get_position(),
            value: TokenValue::None,
        });
    }

    fn read_equals(&self, state: &mut ReaderState) -> ReaderResult {
        state.read();

        if state.peek() == Some(&'=') {
            state.read();

            return ReaderResult::Token(Token {
                kind: TokenKind::EqualsEquals,
                start: state.get_start(),
                end: state.get_position(),
                value: TokenValue::None,
            });
        }

        return ReaderResult::Token(Token {
            kind: TokenKind::Equals,
            start: state.get_start(),
            end: state.get_position(),
            value: TokenValue::None,
        });
    }
}

impl Reader for OperatorReader {
    fn name(&self) -> String {
        "OperatorReader".to_string()
    }

    fn read(&self, state: &mut ReaderState) -> ReaderResult {
        match state.peek().unwrap() {
            '=' => self.read_equals(state),
            '-' => self.get_readers_result(TokenKind::Minus, state), // TODO: Handle eq
            '*' => self.get_readers_result(TokenKind::Star, state), // TODO: Handle eq
            '/' => self.get_readers_result(TokenKind::Slash, state), // TODO: Handle eq
            '+' => self.get_readers_result(TokenKind::Plus, state), // TODO: Handle eq
            '%' => self.get_readers_result(TokenKind::Percent, state), // TODO: Handle eq
            '^' => self.get_readers_result(TokenKind::Caret, state), // TODO: Handle eq
            '&' => self.get_readers_result(TokenKind::Ampersand, state), // TODO: Handle && &=
            '|' => self.get_readers_result(TokenKind::Pipe, state), // TODO: Handle || |=
            '!' => self.get_readers_result(TokenKind::Bang, state), // TODO: Handle eq
            '<' => self.get_readers_result(TokenKind::LessThan, state), // TODO: Handle << <= <<=
            '>' => self.get_readers_result(TokenKind::GreaterThan, state), // TODO: Handle >> >= >>=
            '.' => self.get_readers_result(TokenKind::Dot, state), // TODO: Handle ..
            ',' => self.get_readers_result(TokenKind::Comma, state),
            '(' => self.get_readers_result(TokenKind::BraceRoundOpen, state),
            ')' => self.get_readers_result(TokenKind::BraceRoundClose, state),
            '{' => self.get_readers_result(TokenKind::BraceCurlyOpen, state),
            '}' => self.get_readers_result(TokenKind::BraceCurlyClose, state),
            '[' => self.get_readers_result(TokenKind::BraceSquareOpen, state),
            ']' => self.get_readers_result(TokenKind::BraceSquareClose, state),
            _ => ReaderResult::None,
        }
    }
}

struct NewLineReader;

impl Reader for NewLineReader {
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

impl Reader for WhitespaceReader {
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

impl Reader for UnexpectedCharacterReader {
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
        .add_reader(NumberReader)
        .add_reader(OperatorReader)
        .add_reader(NewLineReader)
        .add_reader(WhitespaceReader)
        .add_reader(UnexpectedCharacterReader);

    let string =
        "\
        Ident ident ident_snake identCamel ident123
        123 123.456 123. 123.456
        + - * / % ^ & | && || ! .. < << > >>
        ( ) { } [ ]
        = == += -= *= /= %= ^= <= <<= >= >>= &= &&= |= ||= !=
        . ,
        if else while for loop break
        $ echo \"Hello World!\"
        $ echo Multi \
          line \
          command
        ";

    let result = lexer.lex(&string);

    println!("{result:#?}");
}
