#[derive(Debug, PartialEq)]
pub enum TokenKind {
    NewLine,
    Identifier,
    String,
    Number,
    Command,
    Equals,
    Dot,
    Bang,
    Plus,
    Minus,
    Slash,
    Star,
    Hat,
    Percent,
    Comma,
    Comment,
    BraceCurlyOpen,
    BraceCurlyClose,
    BraceSquareOpen,
    BraceSquareClose,
    BraceRoundOpen,
    BraceRoundClose,
    If,
    Else,
    For,
    While,
    Whitespace,
}

#[derive(Debug, PartialEq)]
pub enum TokenValue {
    None,
    String(String),
    Number(f64),
    Boolean(bool),
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub start: usize,
    pub end: usize,
    pub value: TokenValue,
}
