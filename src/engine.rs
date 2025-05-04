mod lexer;
mod parser;
mod types;

use super::engine::{
    lexer::Lexer,
    parser::Parser,
    types::{Expr, Operator},
};

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
                        let mut result: u128 = 1;
                        let mut i: u128 = n as u128;

                        while i > 0 {
                            result *= i as u128;
                            if i <= amount as u128 {
                                break;
                            }
                            i -= amount as u128;
                        }

                        result as f64
                    }
                }
                _ => unreachable!(),
            }
        }
    }
}
