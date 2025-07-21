mod identifier;
mod lexer;
mod parser;
mod types;

use super::eval::{
    lexer::Lexer,
    parser::Parser,
    types::{Expr, Operator},
};

use num_bigint::BigUint;
use num_traits::{One, ToPrimitive, Zero};

pub fn expr(input: &str) -> Result<f64, String> {
    Ok(calculate(Parser::new(&Lexer::new(input).lex()?).parse()?))
}

fn calculate(expr: Expr) -> f64 {
    match expr {
        Expr::Num(n) => n,
        Expr::Function(id, args) => {
            let mut nums = Vec::new();
            for arg in args.iter() {
                nums.push(calculate((**arg).clone()))
            }

            match id.as_str() {
                "sqrt" => nums[0].sqrt(),
                "ln" => nums[0].ln(),
                "root" => nums[0].powf(1.0 / nums[1]),
                "log" => nums[1].log(nums[0]),
                "cbrt" => nums[0].powf(1.0 / 3.0),

                "sin" => nums[0].sin(),
                "cos" => nums[0].cos(),
                "tan" => nums[0].tan(),
                "cot" => 1.0 / nums[0].sin(),
                "sec" => 1.0 / nums[0].cos(),
                "csc" => 1.0 / nums[0].tan(),

                "asin" => nums[0].asin(),
                "acos" => nums[0].acos(),
                "atan" => nums[0].atan(),
                "acot" => 1.0 / nums[0].atan(),
                "asec" => 1.0 / nums[0].acos(),
                "acsc" => 1.0 / nums[0].asin(),

                "sinh" => nums[0].sinh(),
                "cosh" => nums[0].cosh(),
                "tanh" => nums[0].tanh(),
                "coth" => 1.0 / nums[0].tanh(),
                "sech" => 1.0 / nums[0].cosh(),
                "csch" => 1.0 / nums[0].sinh(),

                "asinh" => nums[0].asinh(),
                "acosh" => nums[0].acosh(),
                "atanh" => nums[0].atanh(),
                "acoth" => 1.0 / nums[0].atanh(),
                "asech" => 1.0 / nums[0].acosh(),
                "acsch" => 1.0 / nums[0].asinh(),
                _ => unreachable!(),
            }
        }
        Expr::Binary(left, op, right) => {
            let l = calculate(*left);
            let r = calculate(*right);

            match op {
                Operator::Addition => l + r,
                Operator::Subtraction => l - r,
                Operator::Multiplication => l * r,
                Operator::Division => l / r,
                Operator::Exponent => l.powf(r),
                Operator::Percent => l * r / 100.0,
                Operator::Equal => {
                    if l == r {
                        1.0
                    } else {
                        0.0
                    }
                }
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

                    let n_u128 = n as u128;
                    let amt_u128 = amount as u128;

                    let mut result = BigUint::one();
                    let mut i = BigUint::from(n_u128);

                    while i > BigUint::zero() {
                        result *= &i;

                        if &i <= &BigUint::from(amt_u128) {
                            break;
                        }

                        i -= BigUint::from(amt_u128);
                    }

                    // Convert to f64 for return, with potential loss of precision
                    result.to_f64().unwrap_or(f64::INFINITY)
                }
                Operator::Absolute => n.abs(),
                _ => unreachable!(),
            }
        }
        Expr::Variable(_id, value) => calculate(*value),
    }
}
