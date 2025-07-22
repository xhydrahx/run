use std::{iter::Peekable, slice::Iter};

use crate::eval::{
    parser::{function, variable},
    types::{Expr, Token},
};

pub fn parse(tokens: &mut Peekable<Iter<Token>>, id: &str) -> Result<Expr, String> {
    match id {
        "sqrt" | "ln" | "root" | "log" | "cbrt" | "sin" | "cos" | "tan" | "cot" | "sec" | "csc"
        | "asin" | "acos" | "atan" | "acot" | "asec" | "acsc" | "sinh" | "cosh" | "tanh"
        | "coth" | "sech" | "csch" | "asinh" | "acosh" | "atanh" | "acoth" | "asech" | "acsch" => {
            function::parse(tokens, id)
        }

        _ => variable::parse(tokens, id),
    }
}
