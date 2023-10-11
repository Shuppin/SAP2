extern crate sap;

use sap::lexer::{Lexer, token::TokenKind};

fn main() {

    let input = std::fs::read_to_string("src/grammar.txt").unwrap();

    let now = std::time::Instant::now();

    let mut lex = Lexer::new(&input);
    let mut token = lex.get_next_token();
    let mut i = 1;

    println!("{:?}", token);
    
    while token.kind != TokenKind::Eof {
        token = lex.get_next_token();
        println!("{:?}", token);
        i += 1;
    }

    println!("Processed {} tokens in {}ms", i, now.elapsed().as_millis());

}
