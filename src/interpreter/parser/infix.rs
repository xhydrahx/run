use std::{iter::Peekable, slice::Iter};

use crate::interpreter::{error::CalcError, lexer::Token, parser::Expr};

pub fn parse(tokens: &mut Peekable<Iter<Token>>) -> Result<Expr, CalcError> {
    match tokens.next() {
        Some(Token::Atomic(a)) => Ok(Expr::Atomic(*a)),
        Some(token) => Err(CalcError::Unexpected(format!("Char: '{}': Expected a number", token))),
        None => Err(CalcError::Unexpected("EOF".into())),
    }
}
