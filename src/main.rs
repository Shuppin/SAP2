extern crate sap;

use sap::{
    colours::*,
    errors::error::handle_error,
    lexer::{token::TokenKind, Lexer},
};

const FILE: &str = "src/grammar.txt";

fn main() {
    let input = std::fs::read_to_string(FILE).unwrap();

    let now = std::time::Instant::now();

    let mut lex = Lexer::new(input.chars());
    let mut i = 1;

    // while token.is_ok_and(|token| token.kind != TokenKind::Eof)
    loop {
        let token = lex.get_next_token();
        println!("{:?}", token);
        match token {
            Ok(token) => {
                if token.kind == TokenKind::Eof {
                    break;
                }
            }
            Err(error) => {
                println!("{style_bold}{colour_red}Error {colour_reset}aborting execution due to error{style_reset}");
                handle_error(error, &input, FILE);
                break;
            }
        }

        i += 1;
    }

    println!("Processed {} tokens in {}ms", i, now.elapsed().as_millis());
}
