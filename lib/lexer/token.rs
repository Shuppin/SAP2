use super::Pos;

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
    Int(i32),
    Float(f32),
    Bool(bool),
    // other
    Eof,
    Seperator,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub pos: Pos,
    // Could be useful for error reporting
    // pub span: Range<usize>
}