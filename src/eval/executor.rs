use crate::eval::types::Expr;

pub mod func;
pub mod bin;
pub mod unary;

pub fn calculate(expr: Expr) -> f64 {
    match expr {
        Expr::Num(n) => n,
        Expr::Func(id, args) => func::process(id, args),
        Expr::Bin(left, op, right) => bin::process(left, op, right),
        Expr::Unary(op, side) => unary::process(op, side),
        Expr::Var(_id, value) => calculate(*value),
    }
}
