mod lexer;
mod types;
mod parser;
use super::eval::lexer::Lexer;

pub fn eval(input: &str) {
    let mut lexer = Lexer::new(input);
    println!("=> {:?}", lexer.lex());
}
