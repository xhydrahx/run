use std::{iter::Peekable, slice::Iter};

use crate::eval::{
    parser::{self, delimeter},
    types::{Expr, Operator, Token},
};

pub fn func(tokens: &mut Peekable<Iter<Token>>, id: &str) -> Result<Expr, String> {
    match tokens.next() {
        Some(Token::LeftParen) => match id {
            "root" => {
                let mut radicand = Vec::new();
                while let Some(next_token) = tokens.next() {
                    if next_token == &Token::Comma {
                        break;
                    }

                    radicand.push(next_token.to_owned());
                }
                Ok(Expr::Function(
                    id.to_string(),
                    vec![
                        Box::new(parser::parse(radicand)?),
                        Box::new(delimeter::paren(tokens)?),
                    ],
                ))
            }
            "log" => Ok(Expr::Function(
                id.to_string(),
                vec![
                    Box::new(Expr::Num(10.0)),
                    Box::new(delimeter::paren(tokens)?),
                ],
            )),
            _ => Ok(Expr::Function(
                id.to_string(),
                vec![Box::new(delimeter::paren(tokens)?)],
            )),
        },
        Some(Token::Underscore) => {
            let mut base = Vec::new();
            while let Some(next_token) = tokens.next() {
                if next_token == &Token::LeftParen {
                    break;
                }

                base.push(next_token.to_owned());
            }

            Ok(Expr::Function(
                id.to_string(),
                vec![
                    Box::new(parser::parse(base)?),
                    Box::new(delimeter::paren(tokens)?),
                ],
            ))
        }
        None => Err(
            "Unexpected end of expression: Expected a number, '(', or unary operator before end"
                .into(),
        ),
        token => Err(format!(
            "Unexpected '{}': Expected parenthesis after '{}'",
            token.unwrap(),
            id
        )),
    }
}

pub fn absolute(tokens: &mut Peekable<Iter<Token>>) -> Result<Expr, String> {
    let mut expr = Vec::new();
    while let Some(token) = tokens.next() {
        if token == &Token::Bar {
            break;
        }

        expr.push(token.to_owned());
    }

    match tokens.peek() {
        Some(Token::Num(n)) => {
            tokens.next();

            Ok(Expr::Binary(
                Box::new(Expr::Unary(
                    Operator::Absolute,
                    Box::new(parser::parse(expr)?),
                )),
                Operator::Multiplication,
                Box::new(Expr::Num(*n)),
            ))
        }
        _ => Ok(Expr::Unary(
            Operator::Absolute,
            Box::new(parser::parse(expr)?),
        )),
    }
}
