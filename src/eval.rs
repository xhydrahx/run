mod lexer;
mod parser;
mod types;
use super::eval::lexer::Lexer;
use super::eval::parser::Parser;
use super::eval::types::{Expr, Operator};

pub fn eval(input: &str) {
    let mut lexer = Lexer::new(input);
    let tokens = match lexer.lex() {
        Ok(value) => value,
        Err(e) => {
            eprintln!("=> {}", e);
            return;
        }
    };

    let mut parser = Parser::new(&tokens);
    match parser.parse() {
        Ok(ast) => println!("=> {}", calculate(&ast)),
        Err(e) => eprintln!("=> {}", e),
    }
}

fn calculate(expr: &Expr) -> f64 {
    match expr {
        Expr::Num(n) => *n,
        Expr::BinaryOp(left, op, right) => {
            let l = calculate(left);
            let r = calculate(right);

            match op {
                Operator::Addition => l + r,
                Operator::Subtraction => l - r,
                Operator::Multiplication => l * r,
                Operator::Division => l / r,
                Operator::Exponent => l.powf(r),
            }
        }
        Expr::UnaryOp(op, right) => {
            let r = calculate(right);

            match op {
                Operator::Subtraction => -r,
                _ => unreachable!(),
            }
        }
    }
}
