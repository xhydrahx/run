mod lexer;
mod types;
mod parser;
use super::eval::lexer::Lexer;
use super::eval::parser::Parser;

pub fn eval(input: &str) {
    let mut lexer = Lexer::new(input);
    let tokens = lexer.lex();
    println!("=> {:?}", tokens);
    let binding = tokens.unwrap();
    let mut parser = Parser::new(&binding);
    match parser.parse() {
        Ok(ast) => println!("=> {:?}", ast),
        Err(e) => eprintln!("=> {}", e),
    }
}
