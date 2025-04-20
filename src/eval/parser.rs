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

        while let Some(&token) = self.tokens.peek() {
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
            Some(Token::LeftParen) => Ok(self.paren()?),
            Some(Token::Minus) => match self.tokens.next() {
                Some(Token::Num(n)) => Ok(Expr::UnaryOp(
                    Operator::Subtraction,
                    Box::new(Expr::Num(*n)),
                )),
                Some(Token::LeftParen) => Ok(Expr::UnaryOp(
                    Operator::Subtraction,
                    Box::new(self.paren()?),
                )),
                _ => Err("Unkown unary '-' arguements".into()),
            },
            _ => Err("Unkown infix symbol".into()),
        }
    }

    fn paren(&mut self) -> Result<Expr, String> {
        let mut tokens = Vec::new();
        let mut depth = 1;

        while let Some(token) = self.tokens.next() {
            match token {
                Token::LeftParen => {
                    depth += 1;
                    tokens.push(*token);
                }
                Token::RightParen => {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                    tokens.push(*token);
                }
                _ => tokens.push(*token),
            }
        }

        if depth != 0 {
            return Err("Unclosed parenthesis".into());
        }

        Parser::new(tokens.as_slice()).parse()
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
