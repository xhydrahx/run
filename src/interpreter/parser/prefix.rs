use std::{iter::Peekable, slice::Iter};

use crate::interpreter::{
    error::CalcError,
    lexer::Token,
    parser::{self, Expr, Operator},
};

pub fn parse(
    tokens: &mut Peekable<Iter<Token>>,
    left: Expr,
    token: &Token,
    precedence: u8,
) -> Result<Expr, CalcError> {
    let right = parser::primary(tokens, precedence + 1)?;

    match token {
        Token::Plus => Ok(Expr::Binary(
            Box::new(left),
            Operator::Plus,
            Box::new(right),
        )),
        Token::Minus => Ok(Expr::Binary(
            Box::new(left),
            Operator::Minus,
            Box::new(right),
        )),
        Token::Star => Ok(Expr::Binary(
            Box::new(left),
            Operator::Multiplication,
            Box::new(right),
        )),
        Token::Slash => Ok(Expr::Binary(
            Box::new(left),
            Operator::Division,
            Box::new(right),
        )),
        _ => Err(CalcError::Unexpected(format!(
            "Char: '{}': Expected an operator",
            token
        ))),
    }
}
