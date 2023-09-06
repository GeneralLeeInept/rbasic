mod ast;

use ast::lexer::{Lexer, Token};
use std::io::{self, Write};

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading input");
        input.pop();

        let mut lexer = Lexer::new(&input);
        let mut tokens: Vec<Token> = Vec::new();
        while let Some(token) = lexer.get() {
            tokens.push(token);
        }
        println!("{:?}", tokens);
    }
}
