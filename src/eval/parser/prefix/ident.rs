use std::{iter::Peekable, slice::Iter};

use crate::eval::{
    types::{Expr, Token},
};

pub mod func;
pub mod var;

pub fn parse(tokens: &mut Peekable<Iter<Token>>, id: &str) -> Result<Expr, String> {
    match id {
        "sqrt" | "ln" | "root" | "log" | "cbrt" | "sin" | "cos" | "tan" | "cot" | "sec" | "csc"
        | "asin" | "acos" | "atan" | "acot" | "asec" | "acsc" | "sinh" | "cosh" | "tanh"
        | "coth" | "sech" | "csch" | "asinh" | "acosh" | "atanh" | "acoth" | "asech" | "acsch" => {
            func::parse(tokens, id)
        }

        _ => var::parse(tokens, id),
    }
}
