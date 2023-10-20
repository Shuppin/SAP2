use super::Span;

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    // symbols
    Mult,
    Div,
    Plus,
    Minus,
    Mod,
    Lparen,
    Rparen,
    LBracket,
    RBracket,
    Assign,
    Eq,
    NotEq,
    Less,
    LessEq,
    More,
    MoreEq,
    Comma,
    // keywords
    Import,
    Fn,
    If,
    Elif,
    Else,
    Then,
    While,
    Do,
    Return,
    End,
    And,
    Or,
    Not,
    // literals
    Ident(String),
    String(String),
    Int(i32),
    Float(f32),
    Bool(bool),
    // other
    Eof,
    Seperator,
    Unknown,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn empty() -> Self {
        Self {
            kind: TokenKind::Eof,
            span: Span { start: 0, end: 0 },
        }
    }
}
