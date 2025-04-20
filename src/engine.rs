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
                _ => unreachable!(),
            }
        }
        Expr::Unary(op, side) => {
            let n = calculate(*side);

            match op {
                Operator::Subtraction => -n,
                Operator::Factorial(amount) => {
                    if amount == 0 || n == 0.0 {
                        return 1.0;
                    }

                    if amount == 1 {
                        let mut f: i128 = 1;
                        for i in 1..((n + 1.0) as i64) {
                            f *= i as i128;
                        }
                        f as f64
                    } else {
                        let mut result = 1;
                        let mut i = n;

                        loop {
                            result *= i as i64;
                            if i <= amount as f64 || i > 0.0 {
                                break;
                            }
                            i -= amount as f64;
                        }

                        result as f64
                    }
                }
                _ => unreachable!(),
            }
        }
    }
}
