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

/// Reads one character from the state and asserts that it is equal to the given character when
/// compiled in debug mode.
///
/// # Example
/// ```rs
/// read_char!(state, '=');
/// ```
macro_rules! read_char {
    ($state:ident, $char:expr) => {{
        debug_assert!($state.read() == Some(&$char));
        $char
    }};
}

/// Peeks the next character returning true if it is equal to the given character.
///
/// # Example
/// ```rs
/// if peek_char!(state, '=') {
///    // ...
/// }
macro_rules! peek_char {
    ($state:ident, $char:expr) => {
        ($state.peek() == Some(&$char))
    };
}

struct CommentReader;

impl Reader for CommentReader {
    fn name(&self) -> String {
        "CommentReader".to_string()
    }

    fn read(&self, state: &mut ReaderState) -> ReaderResult {
        if let None = state.read_str("//") {
            return ReaderResult::None;
        }

        state.consume_whitespace();

        let mut value = String::new();

        while let Some(ch) = state.clone().peek() {
            if ch == &'\n' {
                break;
            }
            value.push(read_char!(state, *ch));
        }
        
        return ReaderResult::Token(Token {
            kind: TokenKind::Comment,
            start: state.get_start(),
            end: state.get_position(),
            value: TokenValue::String(value),
        });
    }
}

struct KeywordReader;

impl Reader for KeywordReader {
    fn name(&self) -> String {
        "KeywordReader".to_string()
    }

    fn read(&self, state: &mut ReaderState) -> ReaderResult {
        if let Some(_) = state.read_str("if") {
            return ReaderResult::Token(Token {
                kind: TokenKind::If,
                start: state.get_start(),
                end: state.get_position(),
                value: TokenValue::None,
            });
        }

        if let Some(_) = state.read_str("else") {
            return ReaderResult::Token(Token {
                kind: TokenKind::Else,
                start: state.get_start(),
                end: state.get_position(),
                value: TokenValue::None,
            });
        }

        if let Some(_) = state.read_str("for") {
            return ReaderResult::Token(Token {
                kind: TokenKind::For,
                start: state.get_start(),
                end: state.get_position(),
                value: TokenValue::None,
            });
        }

        if let Some(_) = state.read_str("while") {
            return ReaderResult::Token(Token {
                kind: TokenKind::While,
                start: state.get_start(),
                end: state.get_position(),
                value: TokenValue::None,
            });
        }

        if let Some(_) = state.read_str("loop") {
            return ReaderResult::Token(Token {
                kind: TokenKind::Loop,
                start: state.get_start(),
                end: state.get_position(),
                value: TokenValue::None,
            });
        }

        if let Some(_) = state.read_str("break") {
            return ReaderResult::Token(Token {
                kind: TokenKind::Break,
                start: state.get_start(),
                end: state.get_position(),
                value: TokenValue::None,
            });
        }

        if let Some(_) = state.read_str("continue") {
            return ReaderResult::Token(Token {
                kind: TokenKind::Continue,
                start: state.get_start(),
                end: state.get_position(),
                value: TokenValue::None,
            });
        }

        if let Some(_) = state.read_str("return") {
            return ReaderResult::Token(Token {
                kind: TokenKind::Return,
                start: state.get_start(),
                end: state.get_position(),
                value: TokenValue::None,
            });
        }

        return ReaderResult::None;
    }
}

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

struct StringReader;

impl Reader for StringReader {
    fn name(&self) -> String {
        "StringReader".to_string()
    }

    fn read(&self, state: &mut ReaderState) -> ReaderResult {
        let mut value = String::new();

        // Check if the first character is a double quote
        if !peek_char!(state, '"') {
            return ReaderResult::None;
        }

        read_char!(state, '"');

        // Read all characters until the next unescaped double quote
        while let Some(char) = state.clone().peek() {
            read_char!(state, *char);
            match char {
                '\\' if peek_char!(state, '"') => {
                    value.push(read_char!(state, '"'));
                },
                '"' => break,
                _ => value.push(*char),
            }
        }

        return ReaderResult::Token(Token {
            kind: TokenKind::String,
            start: state.get_start(),
            end: state.get_position(),
            value: TokenValue::String(value),
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
        read_char!(state, '=');

        if peek_char!(state, '=') {
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

    fn read_minus(&self, state: &mut ReaderState) -> ReaderResult {
        read_char!(state, '-');

        if peek_char!(state, '=') {
            state.read();

            return ReaderResult::Token(Token {
                kind: TokenKind::MinusEquals,
                start: state.get_start(),
                end: state.get_position(),
                value: TokenValue::None,
            });
        }

        return ReaderResult::Token(Token {
            kind: TokenKind::Minus,
            start: state.get_start(),
            end: state.get_position(),
            value: TokenValue::None,
        });
    }

    fn read_star(&self, state: &mut ReaderState) -> ReaderResult {
        read_char!(state, '*');

        if peek_char!(state, '=') {
            state.read();

            return ReaderResult::Token(Token {
                kind: TokenKind::StarEquals,
                start: state.get_start(),
                end: state.get_position(),
                value: TokenValue::None,
            });
        }

        return ReaderResult::Token(Token {
            kind: TokenKind::Star,
            start: state.get_start(),
            end: state.get_position(),
            value: TokenValue::None,
        });
    }

    fn read_slash(&self, state: &mut ReaderState) -> ReaderResult {
        read_char!(state, '/');

        if peek_char!(state, '=') {
            state.read();

            return ReaderResult::Token(Token {
                kind: TokenKind::SlashEquals,
                start: state.get_start(),
                end: state.get_position(),
                value: TokenValue::None,
            });
        }

        return ReaderResult::Token(Token {
            kind: TokenKind::Slash,
            start: state.get_start(),
            end: state.get_position(),
            value: TokenValue::None,
        });
    }

    fn read_plus(&self, state: &mut ReaderState) -> ReaderResult {
        read_char!(state, '+');

        if peek_char!(state, '=') {
            state.read();

            return ReaderResult::Token(Token {
                kind: TokenKind::PlusEquals,
                start: state.get_start(),
                end: state.get_position(),
                value: TokenValue::None,
            });
        }

        return ReaderResult::Token(Token {
            kind: TokenKind::Plus,
            start: state.get_start(),
            end: state.get_position(),
            value: TokenValue::None,
        });
    }

    fn read_percent(&self, state: &mut ReaderState) -> ReaderResult {
        read_char!(state, '%');

        if peek_char!(state, '=') {
            state.read();

            return ReaderResult::Token(Token {
                kind: TokenKind::PercentEquals,
                start: state.get_start(),
                end: state.get_position(),
                value: TokenValue::None,
            });
        }

        return ReaderResult::Token(Token {
            kind: TokenKind::Percent,
            start: state.get_start(),
            end: state.get_position(),
            value: TokenValue::None,
        });
    }

    fn read_caret(&self, state: &mut ReaderState) -> ReaderResult {
        read_char!(state, '^');

        if peek_char!(state, '=') {
            state.read();

            return ReaderResult::Token(Token {
                kind: TokenKind::CaretEquals,
                start: state.get_start(),
                end: state.get_position(),
                value: TokenValue::None,
            });
        }

        return ReaderResult::Token(Token {
            kind: TokenKind::Caret,
            start: state.get_start(),
            end: state.get_position(),
            value: TokenValue::None,
        });
    }

    fn read_ampersand(&self, state: &mut ReaderState) -> ReaderResult {
        read_char!(state, '&');

        match state.peek() {
            Some('&') => {
                state.read();

                return ReaderResult::Token(Token {
                    kind: TokenKind::AmpersandAmpersand,
                    start: state.get_start(),
                    end: state.get_position(),
                    value: TokenValue::None,
                });
            }
            Some('=') => {
                state.read();

                return ReaderResult::Token(Token {
                    kind: TokenKind::AmpersandEquals,
                    start: state.get_start(),
                    end: state.get_position(),
                    value: TokenValue::None,
                });
            }
            _ => {
                return ReaderResult::Token(Token {
                    kind: TokenKind::Ampersand,
                    start: state.get_start(),
                    end: state.get_position(),
                    value: TokenValue::None,
                });
            }
        }
    }

    fn read_pipe(&self, state: &mut ReaderState) -> ReaderResult {
        read_char!(state, '|');

        match state.peek() {
            Some('|') => {
                state.read();

                return match state.peek() {
                    Some('=') => {
                        state.read();

                        ReaderResult::Token(Token {
                            kind: TokenKind::PipePipeEquals,
                            start: state.get_start(),
                            end: state.get_position(),
                            value: TokenValue::None,
                        })
                    }
                    _ => ReaderResult::Token(Token {
                        kind: TokenKind::PipePipe,
                        start: state.get_start(),
                        end: state.get_position(),
                        value: TokenValue::None,
                    }),
                };
            }
            Some('=') => {
                state.read();

                return ReaderResult::Token(Token {
                    kind: TokenKind::PipeEquals,
                    start: state.get_start(),
                    end: state.get_position(),
                    value: TokenValue::None,
                });
            }
            _ => {
                return ReaderResult::Token(Token {
                    kind: TokenKind::Pipe,
                    start: state.get_start(),
                    end: state.get_position(),
                    value: TokenValue::None,
                });
            }
        }
    }

    fn read_bang(&self, state: &mut ReaderState) -> ReaderResult {
        read_char!(state, '!');

        if peek_char!(state, '=') {
            state.read();

            return ReaderResult::Token(Token {
                kind: TokenKind::BangEquals,
                start: state.get_start(),
                end: state.get_position(),
                value: TokenValue::None,
            });
        }

        return ReaderResult::Token(Token {
            kind: TokenKind::Bang,
            start: state.get_start(),
            end: state.get_position(),
            value: TokenValue::None,
        });
    }

    fn read_less_than(&self, state: &mut ReaderState) -> ReaderResult {
        read_char!(state, '<');

        match state.peek() {
            Some('<') => {
                state.read();

                return match state.peek() {
                    Some('=') => {
                        state.read();

                        ReaderResult::Token(Token {
                            kind: TokenKind::LessThanLessThanEquals,
                            start: state.get_start(),
                            end: state.get_position(),
                            value: TokenValue::None,
                        })
                    }
                    _ => ReaderResult::Token(Token {
                        kind: TokenKind::LessThanLessThan,
                        start: state.get_start(),
                        end: state.get_position(),
                        value: TokenValue::None,
                    }),
                };
            }
            Some('=') => {
                state.read();

                return ReaderResult::Token(Token {
                    kind: TokenKind::LessThanEquals,
                    start: state.get_start(),
                    end: state.get_position(),
                    value: TokenValue::None,
                });
            }
            _ => {
                return ReaderResult::Token(Token {
                    kind: TokenKind::LessThan,
                    start: state.get_start(),
                    end: state.get_position(),
                    value: TokenValue::None,
                });
            }
        }
    }

    fn read_greater_than(&self, state: &mut ReaderState) -> ReaderResult {
        read_char!(state, '>');

        match state.peek() {
            Some('>') => {
                state.read();

                return match state.peek() {
                    Some('=') => {
                        state.read();

                        ReaderResult::Token(Token {
                            kind: TokenKind::GreaterThanGreaterThanEquals,
                            start: state.get_start(),
                            end: state.get_position(),
                            value: TokenValue::None,
                        })
                    }
                    _ => ReaderResult::Token(Token {
                        kind: TokenKind::GreaterThanGreaterThan,
                        start: state.get_start(),
                        end: state.get_position(),
                        value: TokenValue::None,
                    }),
                };
            }
            Some('=') => {
                state.read();

                return ReaderResult::Token(Token {
                    kind: TokenKind::GreaterThanEquals,
                    start: state.get_start(),
                    end: state.get_position(),
                    value: TokenValue::None,
                });
            }
            _ => {
                return ReaderResult::Token(Token {
                    kind: TokenKind::GreaterThan,
                    start: state.get_start(),
                    end: state.get_position(),
                    value: TokenValue::None,
                });
            }
        }
    }

    fn read_dot(&self, state: &mut ReaderState) -> ReaderResult {
        read_char!(state, '.');

        match state.peek() {
            Some('.') => {
                state.read();

                return ReaderResult::Token(Token {
                    kind: TokenKind::DotDot,
                    start: state.get_start(),
                    end: state.get_position(),
                    value: TokenValue::None,
                });
            }
            _ => {
                return ReaderResult::Token(Token {
                    kind: TokenKind::Dot,
                    start: state.get_start(),
                    end: state.get_position(),
                    value: TokenValue::None,
                });
            }
        }
    }
}

impl Reader for OperatorReader {
    fn name(&self) -> String {
        "OperatorReader".to_string()
    }

    fn read(&self, state: &mut ReaderState) -> ReaderResult {
        match state.peek().unwrap() {
            '=' => self.read_equals(state),
            '-' => self.read_minus(state),
            '*' => self.read_star(state),
            '/' => self.read_slash(state),
            '+' => self.read_plus(state),
            '%' => self.read_percent(state),
            '^' => self.read_caret(state),
            '&' => self.read_ampersand(state),
            '|' => self.read_pipe(state),
            '!' => self.read_bang(state),
            '<' => self.read_less_than(state),
            '>' => self.read_greater_than(state),
            '.' => self.read_dot(state),
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

struct CommandReader;

impl Reader for CommandReader {
    fn name(&self) -> String {
        "CommandReader".to_string()
    }

    fn read(&self, state: &mut ReaderState) -> ReaderResult {
        if !peek_char!(state, '$') {
            return ReaderResult::None;
        }

        read_char!(state, '$');

        state.consume_whitespace();

        let mut command = String::new();

        while let Some(char) = state.clone().peek() {
            match char {
                // Escape new lines
                '\\' if peek_char!(state, '\n') => {
                    read_char!(state, '\\');
                    command.push(read_char!(state, '\n'));
                    continue;
                }
                // Unescaped newline ends the command
                '\n' => break,
                // All other characters are part of the command
                char => {
                    command.push(read_char!(state, *char));
                }
            }
        }

        return ReaderResult::Token(Token {
            kind: TokenKind::Command,
            start: state.get_start(),
            end: state.get_position(),
            value: TokenValue::String(command),
        });
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
        .add_reader(CommentReader)
        .add_reader(KeywordReader)
        .add_reader(IdentifierReader)
        .add_reader(NumberReader)
        .add_reader(StringReader)
        .add_reader(OperatorReader)
        .add_reader(CommandReader)
        .add_reader(NewLineReader)
        .add_reader(WhitespaceReader)
        .add_reader(UnexpectedCharacterReader);

    let source = "\
        Ident ident ident_snake identCamel ident123
        123 123.456 123. 123.456
        \"\" \"Hello World\" \"Hello \\\"World\\\"!\" \"multi
        line
        string\"
        true false
        + - * / % ^ & | && || ! .. < << > >>
        ( ) { } [ ]
        = == += -= *= /= %= ^= <= <<= >= >>= &= &&= |= ||= !=
        . ,
        if else while for loop break continue return
        $ echo \"Hello World!\"
        $ echo Multi \
               line \
               command

        // Uses the echo command to say hello to the name passed in
        sayHello = (name) {
            result = $ echo \"Hello $name!\"
            return result.code
        }

        main = () {
            return sayHello(\"World\")
        }
        ";

    let result = lexer.lex(&source);

    // println!("{result:#?}");

    result.errors.iter().for_each(move |error| {
        println!("{}", error.format_inline(source));
    });
}
