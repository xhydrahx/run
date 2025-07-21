pub mod executor;
pub mod identifier;
pub mod lexer;
pub mod parser;
pub mod types;

pub fn evaluate(input: &str) -> Result<f64, String> {
    Ok(executor::calculate(
        parser::Parser::new(&lexer::Lexer::new(input).lex()?).parse()?,
    ))
}
