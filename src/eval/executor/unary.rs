use num_bigint::BigUint;
use num_traits::{One, ToPrimitive, Zero};

use crate::eval::{
    executor,
    types::{Expr, Operator},
};

pub fn process(op: Operator, side: Box<Expr>) -> f64 {
    let n = executor::calculate(*side);

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
