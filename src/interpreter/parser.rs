use std::{iter::Peekable, slice::Iter};

use crate::interpreter::{error::CalcError, lexer::Token};

mod infix;
mod prefix;

#[derive(Debug, PartialEq)]
pub enum Operator {
    Plus,
    Minus,
    Multiplication,
    Division,
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Atomic(f64),
    Binary(Box<Expr>, Operator, Box<Expr>),
}

pub fn primary(tokens: &mut Peekable<Iter<Token>>, precedence: u8) -> Result<Expr, CalcError> {
    let mut left = infix::parse(tokens)?;

    while let Some(token) = tokens.next() {
        let token_precedence = get_precedence(token.to_owned());

        if token_precedence < precedence {
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

#[cfg(test)]

mod tests {
    use crate::interpreter::lexer::lex;

    use super::*;

    #[test]
    fn test_parse() {
        let tokens = lex("9+10".to_string()).expect("Could not lex");
        let expr = primary(&mut tokens.iter().peekable(), 0).expect("Could not parse");
        let expected_expr = Expr::Binary(
            Box::new(Expr::Atomic(9.0)),
            Operator::Plus,
            Box::new(Expr::Atomic(10.0)),
        );

        assert_eq!(expr, expected_expr);
    }

    #[test]
    fn test_multiple_ops() {
        let tokens = lex("9+10*10/2-1".to_string()).expect("Could not lex");
        let expr = primary(&mut tokens.iter().peekable(), 0).expect("Could not parse");
        let expected_expr = Expr::Binary(
            Box::new(Expr::Binary(
                Box::new(Expr::Atomic(9.0)),
                Operator::Plus,
                Box::new(Expr::Binary(
                    Box::new(Expr::Atomic(10.0)),
                    Operator::Multiplication,
                    Box::new(Expr::Atomic(10.0)),
                )),
            )),
            Operator::Minus,
            Box::new(Expr::Atomic(1.0)),
        );

        assert_eq!(expr, expected_expr);
    }
}
