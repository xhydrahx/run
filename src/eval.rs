use lexer::Lexer;

mod lexer;
mod types;

pub fn run<'a>(input: &'a str) {
    let mut lexer = Lexer::new(input);
    println!("{:?}", lexer.lex());
}
