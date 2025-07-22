use std::{iter::Peekable, slice::Iter};

use crate::eval::{
    parser::{delimeter, identifier, number},
    types::{Expr, Operator, Token},
};

pub fn prefix(tokens: &mut Peekable<Iter<Token>>) -> Result<Expr, String> {
    match tokens.next() {
            Some(Token::Num(n)) => number::num(tokens, *n),
            Some(Token::LeftParen) => Ok(delimeter::paren(tokens)?),
            Some(Token::Minus) => match tokens.next() {
                Some(Token::Num(n)) => Ok(Expr::Unary(Operator::Subtraction, Box::new(number::num(tokens, *n)?))),
                Some(Token::LeftParen) => Ok(Expr::Unary(Operator::Subtraction, Box::new(delimeter::paren(tokens)?))),
                Some(Token::Identifier(id)) => Ok(Expr::Unary(Operator::Subtraction, Box::new(identifier::ident(tokens, id)?))),
                Some(token) => Err(format!("Unexpected token '{}' after unary '-': Expected a number, an opening parenthesis '(', or a valid unary expression.", token)),
                None => Err("Unexpected end of expression: Expected a number, '(', or unary operator before end.".into()),
            },
            Some(Token::Identifier(id)) => identifier::ident(tokens, id),
            Some(Token::Bar) => identifier::absolute(tokens),
            Some(token) => Err(format!(
                "Unexpected token '{}' encountered: Expected a number, an opening parenthesis '(', or a unary operator.",
                token
            )),
            None => Err("Unexpected end of expression: Expected a number, '(', or unary operator before end.".into()),
        }
}
