use std::{iter::Peekable, slice::Iter};

use crate::eval::{
    parser::{delimeter, primary},
    types::{Expr, Operator, Token},
};

pub fn parse(tokens: &mut Peekable<Iter<Token>>, left: Expr) -> Result<Expr, String> {
    let token = tokens.next().unwrap();
    match token {
        Token::Plus => {
            let right = primary(tokens, token.precedence() + 1)?;
            Ok(Expr::Bin(
                Box::new(left),
                Operator::Addition,
                Box::new(right),
            ))
        }
        Token::Minus => {
            let right = primary(tokens, token.precedence() + 1)?;
            Ok(Expr::Bin(
                Box::new(left),
                Operator::Subtraction,
                Box::new(right),
            ))
        }
        Token::Star => {
            let right = primary(tokens, token.precedence() + 1)?;
            Ok(Expr::Bin(
                Box::new(left),
                Operator::Multiplication,
                Box::new(right),
            ))
        }
        Token::Slash => {
            let right = primary(tokens, token.precedence() + 1)?;
            Ok(Expr::Bin(
                Box::new(left),
                Operator::Division,
                Box::new(right),
            ))
        }
        Token::Carrot => {
            let right = primary(tokens, token.precedence())?;
            Ok(Expr::Bin(
                Box::new(left),
                Operator::Exponent,
                Box::new(right),
            ))
        }
        Token::Exclamation => {
            let mut amount: i8 = 1;
            while let Some(token) = tokens.peek() {
                match token {
                    Token::Exclamation => {
                        tokens.next();
                        amount += 1;
                    }
                    _ => break,
                }
            }

            Ok(Expr::Unary(Operator::Factorial(amount), Box::new(left)))
        }
        Token::LeftParen => Ok(Expr::Bin(
            Box::new(left),
            Operator::Multiplication,
            Box::new(delimeter::paren(tokens)?),
        )),
        Token::Percent => match left {
            Expr::Num(n) => Ok(Expr::Bin(
                Box::new(Expr::Num(1.0)),
                Operator::Percent,
                Box::new(Expr::Num(n)),
            )),
            Expr::Bin(l, op, r) => Ok(Expr::Bin(
                l.clone(),
                op,
                Box::new(Expr::Bin(l, Operator::Percent, r)),
            )),
            Expr::Unary(op, r) => Ok(Expr::Unary(
                op,
                Box::new(Expr::Bin(r.clone(), Operator::Percent, r)),
            )),
            Expr::Func(id, args) => Ok(Expr::Bin(
                Box::new(Expr::Func(id.clone(), args.clone())),
                Operator::Percent,
                Box::new(Expr::Func(id, args)),
            )),
            Expr::Var(_, value) => Ok(Expr::Bin(
                Box::new(Expr::Num(1.0)),
                Operator::Percent,
                value,
            )),
        },
        Token::Equal => {
            let right = primary(tokens, 0)?;
            Ok(Expr::Bin(
                Box::new(left),
                Operator::Equal,
                Box::new(right),
            ))
        }
        token => Err(format!(
            "Unknown operator '{}': Expected a valid known operator",
            token
        )),
    }
}
