use super::types::{Expr, Operator, Token};
use std::{iter::Peekable, slice::Iter};

pub struct Parser<'a> {
    tokens: Peekable<Iter<'a, Token>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Parser {
            tokens: tokens.iter().peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<Expr, String> {
        self.primary(0)
    }

    fn primary(&mut self, precedence: u8) -> Result<Expr, String> {
        let mut left = self.prefix()?;

        while let Some(&token) =  self.tokens.peek() {
            if token.precedence() < precedence {
                break;
            }
            left = self.infix(left)?;
        }

        Ok(left)
    }

    fn prefix(&mut self) -> Result<Expr, String> {
        match self.tokens.next() {
            Some(Token::Num(n)) => Ok(Expr::Num(*n)),
            _ => Err("Unkown infix symbol".into()),
        }
    }

    fn infix(&mut self, left: Expr) -> Result<Expr, String> {
        let token = self.tokens.next().unwrap();
        match token {
            Token::Plus => {
                let right = self.primary(token.precedence() + 1)?;
                Ok(Expr::BinaryOp(
                    Box::new(left),
                    Operator::Addition,
                    Box::new(right),
                ))
            }
            Token::Minus => {
                let right = self.primary(token.precedence() + 1)?;
                Ok(Expr::BinaryOp(
                    Box::new(left),
                    Operator::Subtraction,
                    Box::new(right),
                ))
            }
            Token::Star => {
                let right = self.primary(token.precedence() + 1)?;
                Ok(Expr::BinaryOp(
                    Box::new(left),
                    Operator::Multiplication,
                    Box::new(right),
                ))
            }
            Token::Slash => {
                let right = self.primary(token.precedence() + 1)?;
                Ok(Expr::BinaryOp(
                    Box::new(left),
                    Operator::Division,
                    Box::new(right),
                ))
            }
            Token::Carrot => {
                let right = self.primary(token.precedence())?;
                Ok(Expr::BinaryOp(
                    Box::new(left),
                    Operator::Exponent,
                    Box::new(right),
                ))
            }
            _ => Err("Unkown postfix symbol".into()),
        }
    }
}
