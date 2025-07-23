use std::{iter::Peekable, slice::Iter};

use crate::eval::{
    parser::{delimeter, prefix::ident},
    types::{Expr, Operator, Token},
};

pub fn parse(tokens: &mut Peekable<Iter<Token>>, num: f64) -> Result<Expr, String> {
    match tokens.peek() {
        Some(Token::LeftParen) => {
            tokens.next();
            Ok(Expr::Bin(
                Box::new(Expr::Num(num)),
                Operator::Multiplication,
                Box::new(delimeter::paren(tokens)?),
            ))
        }
        Some(Token::Identifier(id)) => {
            tokens.next();
            Ok(Expr::Bin(
                Box::new(Expr::Num(num)),
                Operator::Multiplication,
                Box::new(ident::parse(tokens, id)?),
            ))
        }
        _ => Ok(Expr::Num(num)),
    }
}
