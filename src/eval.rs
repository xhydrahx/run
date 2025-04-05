use lexer::Lexer;
use parser::parse;

mod lexer;
mod types;
mod parser;

pub fn run<'a>(input: &'a str) {
    let mut lexer = Lexer::new(input);
    let tokens = lexer.lex();
    let rpn = parse(tokens.clone());
    println!("Tokens: {:?}", tokens);
    println!("Rpn: {:?}", rpn);
}
