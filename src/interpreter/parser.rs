use std::{iter::Peekable, slice::Iter};

use crate::interpreter::{error::CalcError, lexer::Token};

mod infix;
mod prefix;

pub enum Operator {
    Plus,
    Minus,
    Multiplication,
    Division,
}

pub enum Expr {
    Atomic(f64),
    Binary(Box<Expr>, Operator, Box<Expr>), // Vec[0] = left, Vec[1] h= right.
}

pub fn primary(tokens: &mut Peekable<Iter<Token>>, precedence: u8) -> Result<Expr, CalcError> {
    let mut left = infix::parse(tokens)?;

    while let Some(token) = tokens.next() {
        let token_precedence = get_precedence(token.to_owned());

        if precedence < token_precedence {
            break;
        }

        left = prefix::parse(tokens, left, &token, token_precedence)?;
    }

    Ok(left)
}

pub fn get_precedence(token: Token) -> u8 {
    match token {
        Token::Atomic(_) => 0,
        Token::Plus | Token::Minus => 1,
        Token::Star | Token::Slash => 2,
    }
}
