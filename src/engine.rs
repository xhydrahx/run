mod lexer;
mod parser;
mod types;
use super::engine::lexer::Lexer;
use super::engine::parser::Parser;
use super::engine::types::{Expr, Operator};

pub fn expr(input: &str) -> Result<f64, String> {
    Ok(calculate(Parser::new(&Lexer::new(input).lex()?).parse()?))
}

fn calculate(expr: Expr) -> f64 {
    match expr {
        Expr::Num(n) => n,
        Expr::Binary(left, op, right) => {
            let l = calculate(*left);
            let r = calculate(*right);

            match op {
                Operator::Addition => l + r,
                Operator::Subtraction => l - r,
                Operator::Multiplication => l * r,
                Operator::Division => l / r,
                Operator::Exponent => l.powf(r),
            }
        }
        Expr::Unary(op, right) => {
            let r = calculate(*right);

            match op {
                Operator::Subtraction => -r,
                _ => unreachable!(),
            }
        }
    }
}
