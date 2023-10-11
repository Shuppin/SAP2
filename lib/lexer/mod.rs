pub mod token;
pub mod lexer;
pub use lexer::Lexer;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pos {
    pub line: usize,
    pub col: usize,
}