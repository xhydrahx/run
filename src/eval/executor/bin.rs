use crate::eval::{
    executor,
    types::{Expr, Operator},
};

pub fn process(left: Box<Expr>, op: Operator, right: Box<Expr>) -> f64 {
    let l = executor::calculate(*left);
    let r = executor::calculate(*right);

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
