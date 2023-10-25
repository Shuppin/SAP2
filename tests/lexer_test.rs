// Tests are completely broken due to syntax changes
// TODO: Fix this file

// #[rustfmt::skip]
// #[cfg(test)]
// mod tests {

//     extern crate sap;

//     use sap::lexer::Lexer;
//     use sap::lexer::token::*;
//     use sap::lexer::Pos;

//     // region: Individual tests

//     #[test]
//     fn seperator() {
//         let mut lexer = Lexer::new("a;b \nc \r\nd \r\n\r\ne \r\rf \r\r\ng \n\rh");
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("a".to_string()), pos: Pos { line: 1, col: 1 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Seperator, pos: Pos { line: 1, col: 2 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("b".to_string()), pos: Pos { line: 1, col: 3 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Seperator, pos: Pos { line: 2, col: 1 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("c".to_string()), pos: Pos { line: 2, col: 1 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Seperator, pos: Pos { line: 3, col: 1 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("d".to_string()), pos: Pos { line: 3, col: 1 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Seperator, pos: Pos { line: 5, col: 1 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("e".to_string()), pos: Pos { line: 5, col: 1 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Seperator, pos: Pos { line: 7, col: 1 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("f".to_string()), pos: Pos { line: 7, col: 1 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Seperator, pos: Pos { line: 9, col: 1 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("g".to_string()), pos: Pos { line: 9, col: 1 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Seperator, pos: Pos { line: 11, col: 1 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("h".to_string()), pos: Pos { line: 11, col: 1 } });
//     }

//     #[test]
//     fn garbage() {
//         let mut lexer = Lexer::new("   a/*Comment*/b   // Comment\nc");
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("a".to_string()), pos: Pos { line: 1, col: 4 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("b".to_string()), pos: Pos { line: 1, col: 16 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("c".to_string()), pos: Pos { line: 2, col: 1 } });
//     }

//     #[test]
//     fn identifier() {
//         let mut lexer = Lexer::new("these _are so_me variables _ _6 wys2 ");
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("these".to_string()), pos: Pos { line: 1, col: 1 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("_are".to_string()), pos: Pos { line: 1, col: 7 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("so_me".to_string()), pos: Pos { line: 1, col: 12 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("variables".to_string()), pos: Pos { line: 1, col: 18 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("_".to_string()), pos: Pos { line: 1, col: 28 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("_6".to_string()), pos: Pos { line: 1, col: 30 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("wys2".to_string()), pos: Pos { line: 1, col: 33 } });
//     }

//     #[test]
//     fn keywords() {
//         let mut lexer = Lexer::new("import fn if else then while do return end and or not");
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Import, pos: Pos { line: 1, col: 1 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Fn, pos: Pos { line: 1, col: 8 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::If, pos: Pos { line: 1, col: 11 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Else, pos: Pos { line: 1, col: 14 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Then, pos: Pos { line: 1, col: 19 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::While, pos: Pos { line: 1, col: 24 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Do, pos: Pos { line: 1, col: 30 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Return, pos: Pos { line: 1, col: 33 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::End, pos: Pos { line: 1, col: 40 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::And, pos: Pos { line: 1, col: 44 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Or, pos: Pos { line: 1, col: 48 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Not, pos: Pos { line: 1, col: 51 } });
//     }

//     #[test]
//     fn numbers() {
//         let mut lexer = Lexer::new("1 23 456 3.14 3.0 0.1");
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Int(1), pos: Pos { line: 1, col: 1 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Int(23), pos: Pos { line: 1, col: 3 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Int(456), pos: Pos { line: 1, col: 6 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Float(3.14), pos: Pos { line: 1, col: 10 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Float(3.0), pos: Pos { line: 1, col: 15 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Float(0.1), pos: Pos { line: 1, col: 19 } });
//     }

//     #[test]
//     fn symbols() {
//         let src = "* / + - % ( ) [ ] , ; = == != < <= > >= ";
//         let mut lexer = Lexer::new(src);
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Mult, pos: Pos { line: 1, col: 1 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Div, pos: Pos { line: 1, col: 3 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Plus, pos: Pos { line: 1, col: 5 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Minus, pos: Pos { line: 1, col: 7 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Mod, pos: Pos { line: 1, col: 9 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Lparen, pos: Pos { line: 1, col: 11 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Rparen, pos: Pos { line: 1, col: 13 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::LBracket, pos: Pos { line: 1, col: 15 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::RBracket, pos: Pos { line: 1, col: 17 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Comma, pos: Pos { line: 1, col: 19 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Seperator, pos: Pos { line: 1, col: 21 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Assign, pos: Pos { line: 1, col: 23 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Eq, pos: Pos { line: 1, col: 25 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::NotEq, pos: Pos { line: 1, col: 28 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Less, pos: Pos { line: 1, col: 31 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::LessEq, pos: Pos { line: 1, col: 33 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::More, pos: Pos { line: 1, col: 36 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::MoreEq, pos: Pos { line: 1, col: 38 } });

//     }

//     #[test]
//     fn string_test() {
//         let src = "\"Hello, World!\" \"Hello\\n world\"";
//         let mut lexer = Lexer::new(src);
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::String("Hello, World!".to_string()), pos: Pos { line: 1, col: 1 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::String("Hello\\n world".to_string()), pos: Pos { line: 1, col: 17 } });
//     }

//     // endregion

//     // region: Combined tests

//     #[test]
//     fn assignment() {
//         let src = "x = 10;PI = 3.14";
//         let mut lexer = Lexer::new(src);
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("x".to_string()), pos: Pos { line: 1, col: 1 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Assign, pos: Pos { line: 1, col: 3 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Int(10), pos: Pos { line: 1, col: 5 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Seperator, pos: Pos { line: 1, col: 7 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("PI".to_string()), pos: Pos { line: 1, col: 8 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Assign, pos: Pos { line: 1, col: 11 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Float(3.14), pos: Pos { line: 1, col: 13 } });
//     }

//     #[test]
//     fn basic_arithmetic() {
//         let src = "x = 10; y = 20; z = x + y; a = z - x; b = a * z; c = b / a; d = c % b";
//         let mut lexer = Lexer::new(src);
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("x".to_string()), pos: Pos { line: 1, col: 1 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Assign, pos: Pos { line: 1, col: 3 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Int(10), pos: Pos { line: 1, col: 5 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Seperator, pos: Pos { line: 1, col: 7 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("y".to_string()), pos: Pos { line: 1, col: 9 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Assign, pos: Pos { line: 1, col: 11 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Int(20), pos: Pos { line: 1, col: 13 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Seperator, pos: Pos { line: 1, col: 15 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("z".to_string()), pos: Pos { line: 1, col: 17 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Assign, pos: Pos { line: 1, col: 19 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("x".to_string()), pos: Pos { line: 1, col: 21 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Plus, pos: Pos { line: 1, col: 23 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("y".to_string()), pos: Pos { line: 1, col: 25 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Seperator, pos: Pos { line: 1, col: 26 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("a".to_string()), pos: Pos { line: 1, col: 28 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Assign, pos: Pos { line: 1, col: 30 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("z".to_string()), pos: Pos { line: 1, col: 32 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Minus, pos: Pos { line: 1, col: 34 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("x".to_string()), pos: Pos { line: 1, col: 36 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Seperator, pos: Pos { line: 1, col: 37 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("b".to_string()), pos: Pos { line: 1, col: 39 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Assign, pos: Pos { line: 1, col: 41 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("a".to_string()), pos: Pos { line: 1, col: 43 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Mult, pos: Pos { line: 1, col: 45 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("z".to_string()), pos: Pos { line: 1, col: 47 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Seperator, pos: Pos { line: 1, col: 48 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("c".to_string()), pos: Pos { line: 1, col: 50 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Assign, pos: Pos { line: 1, col: 52 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("b".to_string()), pos: Pos { line: 1, col: 54 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Div, pos: Pos { line: 1, col: 56 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("a".to_string()), pos: Pos { line: 1, col: 58 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Seperator, pos: Pos { line: 1, col: 59 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("d".to_string()), pos: Pos { line: 1, col: 61 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Assign, pos: Pos { line: 1, col: 63 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("c".to_string()), pos: Pos { line: 1, col: 65 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Mod, pos: Pos { line: 1, col: 67 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("b".to_string()), pos: Pos { line: 1, col: 69 } });
//     }

//     #[test]
//     fn functions() {
//         let src = "fn add(a, b) return a+b end; print(add(1, 2))";
//         let mut lexer = Lexer::new(src);
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Fn, pos: Pos { line: 1, col: 1 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("add".to_string()), pos: Pos { line: 1, col: 4 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Lparen, pos: Pos { line: 1, col: 7 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("a".to_string()), pos: Pos { line: 1, col: 8 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Comma, pos: Pos { line: 1, col: 9 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("b".to_string()), pos: Pos { line: 1, col: 11 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Rparen, pos: Pos { line: 1, col: 12 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Return, pos: Pos { line: 1, col: 14 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("a".to_string()), pos: Pos { line: 1, col: 21 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Plus, pos: Pos { line: 1, col: 22 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("b".to_string()), pos: Pos { line: 1, col: 23 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::End, pos: Pos { line: 1, col: 25 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Seperator, pos: Pos { line: 1, col: 28 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("print".to_string()), pos: Pos { line: 1, col: 30 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Lparen, pos: Pos { line: 1, col: 35 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("add".to_string()), pos: Pos { line: 1, col: 36 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Lparen, pos: Pos { line: 1, col: 39 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Int(1), pos: Pos { line: 1, col: 40 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Comma, pos: Pos { line: 1, col: 41 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Int(2), pos: Pos { line: 1, col: 43 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Rparen, pos: Pos { line: 1, col: 44 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Rparen, pos: Pos { line: 1, col: 45 } });

//         let src = "fn sub(a, b)\r\nsum = a - b\r\nreturn sum\r\nend add(1, 2)";
//         let mut lexer = Lexer::new(src);

//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Fn, pos: Pos { line: 1, col: 1 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("sub".to_string()), pos: Pos { line: 1, col: 4 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Lparen, pos: Pos { line: 1, col: 7 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("a".to_string()), pos: Pos { line: 1, col: 8 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Comma, pos: Pos { line: 1, col: 9 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("b".to_string()), pos: Pos { line: 1, col: 11 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Rparen, pos: Pos { line: 1, col: 12 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Seperator, pos: Pos { line: 2, col: 1 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("sum".to_string()), pos: Pos { line: 2, col: 1 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Assign, pos: Pos { line: 2, col: 5 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("a".to_string()), pos: Pos { line: 2, col: 7 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Minus, pos: Pos { line: 2, col: 9 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("b".to_string()), pos: Pos { line: 2, col: 11 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Seperator, pos: Pos { line: 3, col: 1 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Return, pos: Pos { line: 3, col: 1 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("sum".to_string()), pos: Pos { line: 3, col: 8 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Seperator, pos: Pos { line: 4, col: 1 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::End, pos: Pos { line: 4, col: 1 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("add".to_string()), pos: Pos { line: 4, col: 5 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Lparen, pos: Pos { line: 4, col: 8 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Int(1), pos: Pos { line: 4, col: 9 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Comma, pos: Pos { line: 4, col: 10 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Int(2), pos: Pos { line: 4, col: 12 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Rparen, pos: Pos { line: 4, col: 13 } });
//     }

//     #[test]
//     fn comparison_statements() {
//         let src = "if a == b or c != d then\r\n/*do nothing*/\r\nelse e < f and not g > h >= i <= j then end";
//         let mut lexer = Lexer::new(src);
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::If, pos: Pos { line: 1, col: 1 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("a".to_string()), pos: Pos { line: 1, col: 4 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Eq, pos: Pos { line: 1, col: 6 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("b".to_string()), pos: Pos { line: 1, col: 9 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Or, pos: Pos { line: 1, col: 11 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("c".to_string()), pos: Pos { line: 1, col: 14 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::NotEq, pos: Pos { line: 1, col: 16 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("d".to_string()), pos: Pos { line: 1, col: 19 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Then, pos: Pos { line: 1, col: 21 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Seperator, pos: Pos { line: 3, col: 1 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Else, pos: Pos { line: 3, col: 1 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("e".to_string()), pos: Pos { line: 3, col: 6 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Less, pos: Pos { line: 3, col: 8 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("f".to_string()), pos: Pos { line: 3, col: 10 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::And, pos: Pos { line: 3, col: 12 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Not, pos: Pos { line: 3, col: 16 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("g".to_string()), pos: Pos { line: 3, col: 20 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::More, pos: Pos { line: 3, col: 22 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("h".to_string()), pos: Pos { line: 3, col: 24 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::MoreEq, pos: Pos { line: 3, col: 26 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("i".to_string()), pos: Pos { line: 3, col: 29 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::LessEq, pos: Pos { line: 3, col: 31 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("j".to_string()), pos: Pos { line: 3, col: 34 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Then, pos: Pos { line: 3, col: 36 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::End, pos: Pos { line: 3, col: 41 } });

//         let src = "if result then /*Something*/ elif something then /*Something*/ else end";
//         let mut lexer = Lexer::new(src);
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::If, pos: Pos { line: 1, col: 1 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("result".to_string()), pos: Pos { line: 1, col: 4 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Then, pos: Pos { line: 1, col: 11 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Elif, pos: Pos { line: 1, col: 30 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("something".to_string()), pos: Pos { line: 1, col: 35 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Then, pos: Pos { line: 1, col: 45 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Else, pos: Pos { line: 1, col: 64 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::End, pos: Pos { line: 1, col: 69 } });

//     }

//     #[test]
//     fn while_statements() {
//         let src = "while a < b do\r\na = a + 1\r\nend";
//         let mut lexer = Lexer::new(src);
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::While, pos: Pos { line: 1, col: 1 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("a".to_string()), pos: Pos { line: 1, col: 7 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Less, pos: Pos { line: 1, col: 9 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("b".to_string()), pos: Pos { line: 1, col: 11 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Do, pos: Pos { line: 1, col: 13 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Seperator, pos: Pos { line: 2, col: 1 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("a".to_string()), pos: Pos { line: 2, col: 1 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Assign, pos: Pos { line: 2, col: 3 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("a".to_string()), pos: Pos { line: 2, col: 5 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Plus, pos: Pos { line: 2, col: 7 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Int(1), pos: Pos { line: 2, col: 9 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Seperator, pos: Pos { line: 3, col: 1 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::End, pos: Pos { line: 3, col: 1 } });
//     }

//     #[test]
//     fn import_statements() {
//         let src = "import test\r\nimport yes, no";
//         let mut lexer = Lexer::new(src);
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Import, pos: Pos { line: 1, col: 1 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("test".to_string()), pos: Pos { line: 1, col: 8 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Seperator, pos: Pos { line: 2, col: 1 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Import, pos: Pos { line: 2, col: 1 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("yes".to_string()), pos: Pos { line: 2, col: 8 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Comma, pos: Pos { line: 2, col: 11 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("no".to_string()), pos: Pos { line: 2, col: 13 } });
//     }

//     #[test]
//     fn lists() {
//         let src = "x = [1, 2, 3, 4, 5]";
//         let mut lexer = Lexer::new(src);
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Ident("x".to_string()), pos: Pos { line: 1, col: 1 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Assign, pos: Pos { line: 1, col: 3 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::LBracket, pos: Pos { line: 1, col: 5 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Int(1), pos: Pos { line: 1, col: 6 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Comma, pos: Pos { line: 1, col: 7 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Int(2), pos: Pos { line: 1, col: 9 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Comma, pos: Pos { line: 1, col: 10 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Int(3), pos: Pos { line: 1, col: 12 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Comma, pos: Pos { line: 1, col: 13 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Int(4), pos: Pos { line: 1, col: 15 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Comma, pos: Pos { line: 1, col: 16 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::Int(5), pos: Pos { line: 1, col: 18 } });
//         assert_eq!(lexer.get_next_token(), Token { kind: TokenKind::RBracket, pos: Pos { line: 1, col: 19 } });
//     }

//     // endregion
// }
