pub mod executor;
pub mod variables;
pub mod lexer;
pub mod parser;
pub mod types;

pub fn evaluate(expr: &str) -> Result<f64, String> {
    Ok(executor::calculate(
        parser::parse(lexer::lex(&mut expr.chars().peekable())?)?,
    ))
}
