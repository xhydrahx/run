use crate::eval::types::{Expr, Token};
use std::{iter::Peekable, slice::Iter};

pub mod identifier;
pub mod delimeter;
pub mod number;
pub mod prefix;
pub mod function;
pub mod infix;
pub mod variable;

pub fn parse(tokens: Vec<Token>) -> Result<Expr, String> {
    primary(&mut tokens.iter().peekable(), 0)
}

pub fn primary(tokens: &mut Peekable<Iter<Token>>, precedence: u8) -> Result<Expr, String> {
    let mut left = prefix::parse(tokens)?;

    while let Some(&token) = tokens.peek() {
        if token.precedence() < precedence {
            break;
        }
        left = infix::parse(tokens, left)?;
    }

    Ok(left)
}
