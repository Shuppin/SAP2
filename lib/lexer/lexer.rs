use crate::lexer::token::*;

use super::Pos;

type EncountredNewline = bool;

pub struct Lexer {
    input: Vec<char>,
    pos: Pos,
    cur_idx: usize,
    cur: char,
}

impl Lexer {
    pub fn new(input: &str) -> Self {

        // println!("input: {:?}", input);
        let input: Vec<char> = input.chars().collect();
        let cur: char = input.get(0).copied().unwrap_or('\0');

        Self {
            input,
            pos: Pos { line: 1, col: 1 },
            cur_idx: 0,
            cur
        }
    }

    fn advance(&mut self) {
        if self.cur == '\r' {
            // The character sequence '\r\n' is treated as a single newline
            if self.peek() == '\n' {
                self.cur_idx += 1;
            }
            self.pos.line += 1;
            self.pos.col = 1;
        }
        else if self.cur == '\n' {
            self.pos.line += 1;
            self.pos.col = 1;
        } else {
            self.pos.col += 1;
        }

        self.cur_idx += 1;
        self.cur = self.input
            .get(self.cur_idx)
            .copied()
            .unwrap_or('\0');
    }

    fn peek(&self) -> char {
        self.input
            .get(self.cur_idx + 1)
            .copied()
            .unwrap_or('\0')
    }

    fn skip_whitespace(&mut self) {
        while self.cur == ' ' {
            self.advance();
        }
    }

    fn skip_comment(&mut self) {
        // Consume the comment
        while self.cur != '\n' && self.cur != '\r' {
            self.advance();
            if self.cur == '\0' {break};
        }
        // Consume the newline
        self.advance();
    }

    fn skip_multi_comment(&mut self) {
        // Consume the opening '/*'
        if self.cur == '/' && self.peek() == '*' {
            self.advance();
            self.advance();
        }
        else {
            return;
        }
        // Consume the comment
        while !(self.cur == '*' && self.peek() == '/') {
            self.advance();
            if self.cur == '\0' {
                panic!("Unexpected end of file while parsing multiline comment");
            };
        }
        // Consume the closing '*/'
        self.advance();
        self.advance();
    }

    fn skip_garbage(&mut self) -> EncountredNewline {
        // We store whether we encountered a newline because the lexer does
        // count newlines, however it only needs to know if it encountered one,
        // not how many it encountered.
        let mut encountered_newline = false;
        while matches!(self.cur, ' '|'\n'|'\r'|'/') {
            match self.cur {
                // Skip whitespace
                ' ' => self.skip_whitespace(),
                // Skip newlines
                '\n' | '\r' => {
                    encountered_newline = true;
                    self.advance();
                },
                // Skip comments
                '/' => match self.peek() {
                    '/' => self.skip_comment(),
                    '*' => self.skip_multi_comment(),
                    _ => break,
                },
                _ => unreachable!(),
            }
        }
        return encountered_newline;
    }

    fn read_ident(&mut self) -> String {
        let mut ident = String::new();
        // Check if the first character is a letter or an underscore
        if self.cur.is_alphabetic() || self.cur == '_' {
            // After the first letter, the identifier can also contain numbers
            while self.cur.is_alphanumeric() || self.cur == '_'  {
                ident.push(self.cur);
                self.advance();
            }
        }
        else {
            panic!("Invalid identifier: {} - Identifier must start with a letter or an underscore", self.cur);
        }
        return ident;
    }

    fn read_integer(&mut self) -> String {
        let mut num = String::new();
        if self.cur.is_numeric() {
            num.push(self.cur);
            self.advance();
            while self.cur.is_numeric() {
                num.push(self.cur);
                self.advance();
            }
        }
        else {
            panic!("Failed when constrcuting integer: '{}', found '{}'", num, self.cur);
        }
        return num;
    }

    fn read_number(&mut self) -> String {
        let mut num = self.read_integer();
        if self.cur == '.' {
            num.push(self.cur);
            self.advance();
            num.push_str(self.read_integer().as_str());
        }
        return num;
    }

    pub fn get_next_token(&mut self) -> Token {
        while self.cur != '\0' {

            if self.skip_garbage() {
                return Token { kind: TokenKind::Seperator, pos: self.pos };
            }

            let start_pos = self.pos;
            
            if self.cur.is_alphabetic() || self.cur == '_' {
                let ident = self.read_ident();
                let kind = match ident.as_str() {
                    "import" => TokenKind::Import,
                    "fn" => TokenKind::Fn,
                    "if" => TokenKind::If,
                    "elif" => TokenKind::Elif,
                    "else" => TokenKind::Else,
                    "then" => TokenKind::Then,
                    "while" => TokenKind::While,
                    "do" => TokenKind::Do,
                    "return" => TokenKind::Return,
                    "end" => TokenKind::End,
                    "and" => TokenKind::And,
                    "or" => TokenKind::Or,
                    "not" => TokenKind::Not,
                    "true" => TokenKind::Bool(true),
                    "false" => TokenKind::Bool(false),
                    _ => TokenKind::Ident(ident),
                };
                return Token { kind, pos: start_pos };
            }

            else if self.cur.is_numeric() {
                let num = self.read_number();
                let kind = if num.contains('.') {
                    TokenKind::Float(num.parse().unwrap())
                } else {
                    TokenKind::Int(num.parse().unwrap())
                };
                return Token { kind, pos: start_pos };
            }

            else {
                let kind = match self.cur {
                    // Single character symbols
                    '*' => TokenKind::Mult,
                    '/' => TokenKind::Div,
                    '+' => TokenKind::Plus,
                    '-' => TokenKind::Minus,
                    '%' => TokenKind::Mod,
                    '(' => TokenKind::Lparen,
                    ')' => TokenKind::Rparen,
                    '[' => TokenKind::LBracket,
                    ']' => TokenKind::RBracket,
                    ',' => TokenKind::Comma,
                    ';' => TokenKind::Seperator,
                    // Two character symbols
                    '=' => {match self.peek() {
                        '=' => {self.advance();TokenKind::Eq},
                        _ => TokenKind::Assign
                    }},
                    '<' => {match self.peek() {
                        '=' => {self.advance();TokenKind::LessEq},
                        _ => TokenKind::Less
                    }},
                    '>' => {match self.peek() {
                        '=' => {self.advance();TokenKind::MoreEq},
                        _ => TokenKind::More
                    }},
                    '!' => {match self.peek() {
                        '=' => {self.advance();TokenKind::NotEq},
                        _ => panic!("Invalid character: {:?} at {:?}", self.cur, self.pos)
                    }},
                    _ => panic!("Invalid character: {:?} at {:?}", self.cur, self.pos),
                };
                self.advance();
                return Token { kind, pos: start_pos };
            }
        }

        Token { kind: TokenKind::Eof, pos: self.pos }
    }
}
