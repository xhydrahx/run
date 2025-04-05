mod lexer;
mod types;
mod rpn;

pub fn run<'a>(input: &str) {
    let tokens = lexer::lex(input);
    let rpn = rpn::into(tokens.clone());
    println!("Tokens: {:?}", tokens);
    println!("Rpn: {:?}", rpn);
}
