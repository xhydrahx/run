use std::{iter::Peekable, slice::Iter};

use crate::eval::{
    parser::{delimeter, num},
    types::{Expr, Operator, Token},
};

pub mod ident;

pub fn parse(tokens: &mut Peekable<Iter<Token>>) -> Result<Expr, String> {
    match tokens.next() {
            Some(Token::Num(n)) => num::parse(tokens, *n),
            Some(Token::LeftParen) => Ok(delimeter::paren(tokens)?),
            Some(Token::Minus) => match tokens.next() {
                Some(Token::Num(n)) => Ok(Expr::Unary(Operator::Subtraction, Box::new(num::parse(tokens, *n)?))),
                Some(Token::LeftParen) => Ok(Expr::Unary(Operator::Subtraction, Box::new(delimeter::paren(tokens)?))),
                Some(Token::Identifier(id)) => Ok(Expr::Unary(Operator::Subtraction, Box::new(ident::parse(tokens, id)?))),
                Some(token) => Err(format!("Unexpected token '{}' after unary '-': Expected a number, an opening parenthesis '(', or a valid unary expression.", token)),
                None => Err("Unexpected end of expression: Expected a number, '(', or unary operator before end.".into()),
            },
            Some(Token::Identifier(id)) => ident::parse(tokens, id),
            Some(Token::Bar) => ident::func::absolute(tokens),
            Some(token) => Err(format!(
                "Unexpected token '{}' encountered: Expected a number, an opening parenthesis '(', or a unary operator.",
                token
            )),
            None => Err("Unexpected end of expression: Expected a number, '(', or unary operator before end.".into()),
        }
}
