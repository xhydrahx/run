use std::{iter::Peekable, slice::Iter};

use crate::eval::{
    parser::{group, identifier},
    types::{Expr, Operator, Token},
};

pub fn num(tokens: &mut Peekable<Iter<Token>>, num: f64) -> Result<Expr, String> {
    match tokens.peek() {
        Some(Token::LeftParen) => {
            tokens.next();
            Ok(Expr::Binary(
                Box::new(Expr::Num(num)),
                Operator::Multiplication,
                Box::new(group::paren(tokens)?),
            ))
        }
        Some(Token::Identifier(id)) => {
            tokens.next();
            Ok(Expr::Binary(
                Box::new(Expr::Num(num)),
                Operator::Multiplication,
                Box::new(identifier::ident(tokens, id)?),
            ))
        }
        _ => Ok(Expr::Num(num)),
    }
}
