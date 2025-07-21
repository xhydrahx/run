use crate::eval::{
    types::{Expr, Operator, Token},
};
use std::{iter::Peekable, slice::Iter};

pub mod identifier;

pub fn parse(tokens: Vec<Token>) -> Result<Expr, String> {
    primary(&mut tokens.iter().peekable(), 0)
}

pub fn primary(tokens: &mut Peekable<Iter<Token>>, precedence: u8) -> Result<Expr, String> {
    let mut left = prefix(tokens)?;

    while let Some(&token) = tokens.peek() {
        if token.precedence() < precedence {
            break;
        }
        left = infix(tokens, left)?;
    }

    Ok(left)
}

fn prefix(tokens: &mut Peekable<Iter<Token>>) -> Result<Expr, String> {
    match tokens.next() {
            Some(Token::Num(n)) => num(tokens, *n),
            Some(Token::LeftParen) => Ok(paren(tokens)?),
            Some(Token::Minus) => match tokens.next() {
                Some(Token::Num(n)) => Ok(Expr::Unary(Operator::Subtraction, Box::new(num(tokens, *n)?))),
                Some(Token::LeftParen) => Ok(Expr::Unary(Operator::Subtraction, Box::new(paren(tokens)?))),
                Some(Token::Identifier(id)) => Ok(Expr::Unary(Operator::Subtraction, Box::new(identifier::ident(tokens, id)?))),
                Some(token) => Err(format!("Unexpected token '{}' after unary '-': Expected a number, an opening parenthesis '(', or a valid unary expression.", token)),
                None => Err("Unexpected end of expression: Expected a number, '(', or unary operator before end.".into()),
            },
            Some(Token::Identifier(id)) => identifier::ident(tokens, id),
            Some(Token::Bar) => absolute(tokens),
            Some(token) => Err(format!(
                "Unexpected token '{}' encountered: Expected a number, an opening parenthesis '(', or a unary operator.",
                token
            )),
            None => Err("Unexpected end of expression: Expected a number, '(', or unary operator before end.".into()),
        }
}

fn absolute(tokens: &mut Peekable<Iter<Token>>) -> Result<Expr, String> {
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
                Box::new(Expr::Unary(Operator::Absolute, Box::new(parse(expr)?))),
                Operator::Multiplication,
                Box::new(Expr::Num(*n)),
            ))
        }
        _ => Ok(Expr::Unary(Operator::Absolute, Box::new(parse(expr)?))),
    }
}

fn num(tokens: &mut Peekable<Iter<Token>>, num: f64) -> Result<Expr, String> {
    match tokens.peek() {
        Some(Token::LeftParen) => {
            tokens.next();
            Ok(Expr::Binary(
                Box::new(Expr::Num(num)),
                Operator::Multiplication,
                Box::new(paren(tokens)?),
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

fn paren(tokens: &mut Peekable<Iter<Token>>) -> Result<Expr, String> {
    let mut inside = Vec::new();
    let mut depth = 1;

    while let Some(token) = tokens.next() {
        match token {
            Token::LeftParen => {
                depth += 1;
                inside.push(token.to_owned());
            }
            Token::RightParen => {
                depth -= 1;
                if depth == 0 {
                    break;
                }
                inside.push(token.to_owned());
            }
            _ => inside.push(token.to_owned()),
        }
    }

    if depth != 0 {
        return Err(format!(
            "Unclosed parenthesis: {} unmatched '('. Expected {} closing ')' before end of expression.",
            depth, depth
        ));
    }

    parse(inside)
}

fn infix(tokens: &mut Peekable<Iter<Token>>, left: Expr) -> Result<Expr, String> {
    let token = tokens.next().unwrap();
    match token {
        Token::Plus => {
            let right = primary(tokens, token.precedence() + 1)?;
            Ok(Expr::Binary(
                Box::new(left),
                Operator::Addition,
                Box::new(right),
            ))
        }
        Token::Minus => {
            let right = primary(tokens, token.precedence() + 1)?;
            Ok(Expr::Binary(
                Box::new(left),
                Operator::Subtraction,
                Box::new(right),
            ))
        }
        Token::Star => {
            let right = primary(tokens, token.precedence() + 1)?;
            Ok(Expr::Binary(
                Box::new(left),
                Operator::Multiplication,
                Box::new(right),
            ))
        }
        Token::Slash => {
            let right = primary(tokens, token.precedence() + 1)?;
            Ok(Expr::Binary(
                Box::new(left),
                Operator::Division,
                Box::new(right),
            ))
        }
        Token::Carrot => {
            let right = primary(tokens, token.precedence())?;
            Ok(Expr::Binary(
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
        Token::LeftParen => Ok(Expr::Binary(
            Box::new(left),
            Operator::Multiplication,
            Box::new(paren(tokens)?),
        )),
        Token::Percent => match left {
            Expr::Num(n) => Ok(Expr::Binary(
                Box::new(Expr::Num(1.0)),
                Operator::Percent,
                Box::new(Expr::Num(n)),
            )),
            Expr::Binary(l, op, r) => Ok(Expr::Binary(
                l.clone(),
                op,
                Box::new(Expr::Binary(l, Operator::Percent, r)),
            )),
            Expr::Unary(op, r) => Ok(Expr::Unary(
                op,
                Box::new(Expr::Binary(r.clone(), Operator::Percent, r)),
            )),
            Expr::Function(id, args) => Ok(Expr::Binary(
                Box::new(Expr::Function(id.clone(), args.clone())),
                Operator::Percent,
                Box::new(Expr::Function(id, args)),
            )),
            Expr::Variable(_, value) => Ok(Expr::Binary(
                Box::new(Expr::Num(1.0)),
                Operator::Percent,
                value,
            )),
        },
        Token::Equal => {
            let right = primary(tokens, 0)?;
            Ok(Expr::Binary(
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
