use std::fmt::{Display, Debug};

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    NewLine,
    Identifier,
    String,
    Number,
    Boolean,
    Command,
    Equals,
    EqualsEquals,
    BangEquals,
    LessThan,
    LessThanEquals,
    LessThanLessThan,
    LessThanLessThanEquals,
    GreaterThan,
    GreaterThanEquals,
    GreaterThanGreaterThan,
    GreaterThanGreaterThanEquals,
    SlashEquals,
    StarEquals,
    PlusEquals,
    MinusEquals,
    PercentEquals,
    CaretEquals,
    AmpersandEquals,
    AmpersandAmpersandEquals,
    Ampersand,
    AmpersandAmpersand,
    PipeEquals,
    PipePipeEquals,
    Pipe,
    PipePipe,
    Dot,
    DotDot,
    Bang,
    Plus,
    Minus,
    Slash,
    Star,
    Caret,
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
    Loop,
    Break,
    Continue,
    Return,
    Function,
    Whitespace,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenValue {
    None,
    String(String),
    Number(f64),
    Boolean(bool),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub start: usize,
    pub end: usize,
    pub value: TokenValue,
}
