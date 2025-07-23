use crate::eval::types::Expr;

pub mod func;
pub mod bin;
pub mod unary;

pub fn calculate(expr: Expr) -> f64 {
    match expr {
        Expr::Num(n) => n,
        Expr::Function(id, args) => func::process(id, args),
        Expr::Binary(left, op, right) => bin::process(left, op, right),
        Expr::Unary(op, side) => unary::process(op, side),
        Expr::Variable(_id, value) => calculate(*value),
    }
}
