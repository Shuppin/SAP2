use std::str::Chars;

use crate::{
    errors::{Error, ErrorType},
    lexer::token::*,
};

use super::Span;

type EncountredNewline = bool;

pub struct Lexer<'source> {
    input: Chars<'source>,
    cur: char,
    cur_idx: usize,
}

impl<'source> Lexer<'source> {
    pub fn new(mut input: Chars<'source>) -> Self {
        let cur: char = input.next().unwrap_or('\0');

        Self {
            input,
            cur,
            cur_idx: 0,
        }
    }

    fn construct_error(&self, msg: &str, token: Token) -> Result<Token, Error> {
        Err(Error {
            message: msg.to_string(),
            error_type: ErrorType::SyntaxError,
            token,
        })
    }

    fn advance(&mut self) {
        let mut steps = 1;

        if self.cur == '\r' {
            // The character sequence '\r\n' is treated as a single newline
            if self.peek() == '\n' {
                steps += 1;
            }
        }

        self.cur_idx += steps;
        for _ in 0..steps {
            self.cur = self.input.next().unwrap_or('\0');
        }
    }

    fn peek(&self) -> char {
        self.input.clone().next().unwrap_or('\0')
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
            if self.cur == '\0' {
                break;
            };
        }
        // Consume the newline
        self.advance();
    }

    fn skip_multi_comment(&mut self) -> Result<(), Error> {
        let start_pos = self.cur_idx;
        // Consume the opening '/*'
        if self.cur == '/' && self.peek() == '*' {
            self.advance();
            self.advance();
        } else {
            panic!(
                "Invalid multiline comment: {} - Multiline comments must start with '/*'",
                self.cur
            );
        }
        // Consume the comment
        while !(self.cur == '*' && self.peek() == '/') {
            self.advance();
            if self.cur == '\0' {
                return Err(Error {
                    message: "Unexpected end of file while parsing multiline comment".to_string(),
                    error_type: ErrorType::SyntaxError,
                    token: Token {
                        kind: TokenKind::Unknown,
                        span: Span::new(start_pos, start_pos + 2),
                    },
                });
            };
        }
        // Consume the closing '*/'
        self.advance();
        self.advance();

        Ok(())
    }

    fn skip_garbage(&mut self) -> Result<EncountredNewline, Error> {
        // We store whether we encountered a newline because the lexer does
        // count newlines, however it only needs to know if it encountered one,
        // not how many it encountered.
        let mut encountered_newline = false;
        while matches!(self.cur, ' ' | '\n' | '\r' | '/') {
            match self.cur {
                // Skip whitespace
                ' ' => self.skip_whitespace(),
                // Skip newlines
                '\n' | '\r' => {
                    encountered_newline = true;
                    self.advance();
                }
                // Skip comments
                '/' => match self.peek() {
                    '/' => self.skip_comment(),
                    '*' => self.skip_multi_comment()?,
                    _ => break,
                },
                _ => unreachable!(),
            }
        }
        return Ok(encountered_newline);
    }

    fn read_ident(&mut self) -> String {
        let mut ident = String::new();
        // Check if the first character is a letter or an underscore
        if self.cur.is_alphabetic() || self.cur == '_' {
            // After the first letter, the identifier can also contain numbers
            while self.cur.is_alphanumeric() || self.cur == '_' {
                ident.push(self.cur);
                self.advance();
            }
        } else {
            panic!(
                "Invalid identifier: {} - Identifier must start with a letter or an underscore",
                self.cur
            );
        }
        return ident;
    }

    fn read_string(&mut self) -> Result<String, String> {
        let mut string = String::new();
        // Consume the opening '"'
        if self.cur == '"' {
            self.advance();
        } else {
            panic!(
                "Invalid string: {} - String must start with a double quote",
                self.cur
            );
        }
        // Consume the string
        while self.cur != '"' {
            string.push(self.cur);
            self.advance();
            match self.cur {
                '\n' | '\r' => return Err("Unexpected newline while parsing string".to_string()),
                '\0' => return Err("Unexpected end of file while parsing string".to_string()),
                _ => {}
            }
        }
        // Consume the closing '"'
        self.advance();
        return Ok(string);
    }

    fn read_integer(&mut self) -> Result<String, String> {
        let mut num = String::new();
        if self.cur.is_numeric() {
            num.push(self.cur);
            self.advance();
            while self.cur.is_numeric() {
                num.push(self.cur);
                self.advance();
            }
        } else {
            return Err(format!(
                "Failed when constructing integer: '{}', found '{}'",
                num, self.cur
            ));
        }
        return Ok(num);
    }

    fn read_number(&mut self) -> Result<String, String> {
        let mut num = self.read_integer()?;
        if self.cur == '.' {
            num.push(self.cur);
            self.advance();
            num.push_str(self.read_integer()?.as_str());
        }
        return Ok(num);
    }

    pub fn get_next_token(&mut self) -> Result<Token, Error> {
        while self.cur != '\0' {
            if self.skip_garbage()? {
                return Ok(Token {
                    kind: TokenKind::Seperator,
                    span: Span::new(self.cur_idx - 1, self.cur_idx),
                });
            }

            let start_idx = self.cur_idx;

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
                return Ok(Token {
                    kind,
                    span: Span::new(start_idx, self.cur_idx),
                });
            } else if self.cur.is_numeric() {
                let num = match self.read_number() {
                    Ok(num) => num,
                    Err(msg) => {
                        return self.construct_error(
                            msg.as_str(),
                            Token {
                                kind: TokenKind::Unknown,
                                span: Span::new(start_idx, self.cur_idx),
                            },
                        )
                    }
                };
                let kind = if num.contains('.') {
                    TokenKind::Float(num.parse().unwrap())
                } else {
                    TokenKind::Int(num.parse().unwrap())
                };
                return Ok(Token {
                    kind,
                    span: Span::new(start_idx, self.cur_idx),
                });
            } else if self.cur == '"' {
                match self.read_string() {
                    Ok(string) => {
                        return Ok(Token {
                            kind: TokenKind::String(string),
                            span: Span::new(start_idx, self.cur_idx),
                        })
                    }
                    Err(msg) => {
                        return self.construct_error(
                            msg.as_str(),
                            Token {
                                kind: TokenKind::Unknown,
                                span: Span::new(start_idx, self.cur_idx),
                            },
                        )
                    }
                }
            } else {
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
                    '=' => match self.peek() {
                        '=' => {
                            self.advance();
                            TokenKind::Eq
                        }
                        _ => TokenKind::Assign,
                    },
                    '<' => match self.peek() {
                        '=' => {
                            self.advance();
                            TokenKind::LessEq
                        }
                        _ => TokenKind::Less,
                    },
                    '>' => match self.peek() {
                        '=' => {
                            self.advance();
                            TokenKind::MoreEq
                        }
                        _ => TokenKind::More,
                    },
                    '!' => match self.peek() {
                        '=' => {
                            self.advance();
                            TokenKind::NotEq
                        }
                        _ => {
                            return self.construct_error(
                                "Expected '=' after '!'",
                                Token {
                                    kind: TokenKind::Unknown,
                                    span: Span::new(start_idx, start_idx + 2),
                                },
                            )
                        }
                    },
                    _ => {
                        return self.construct_error(
                            "Unexpected symbol",
                            Token {
                                kind: TokenKind::Unknown,
                                span: Span::new(start_idx, start_idx + 1),
                            },
                        )
                    }
                };
                self.advance();
                return Ok(Token {
                    kind,
                    span: Span::new(start_idx, self.cur_idx),
                });
            }
        }

        Ok(Token {
            kind: TokenKind::Eof,
            span: Span::new(self.cur_idx, self.cur_idx),
        })
    }
}
