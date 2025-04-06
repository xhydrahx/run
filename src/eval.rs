use lexer::Lexer;
use parser::Parser;

mod lexer;
mod parser;
mod types;

pub fn run<'a>(input: &'a str) {
    let mut lexer = Lexer::new(input);
    let tokens = lexer.lex();
    let mut parser = Parser::new(&tokens);
    println!("{:?}", tokens);
    println!("{:?}", parser.parse());
}
