use super::types::{Ast, Token};
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

    pub fn parse(&mut self) -> Result<Ast, String> {
        self.expression(0)
    }

    fn expression(&mut self, precedence: u8) -> Result<Ast, String> {
        let mut left = self.primary()?;

        while let Some(&token) = self.tokens.peek() {
            let token_precedence = Self::get_precedence(token);
            if token_precedence < precedence {
                break;
            }

            let token = self.tokens.next().unwrap();
            left = self.infix(left, &token)?;
        }

        Ok(left)
    }

    fn primary(&mut self) -> Result<Ast, String> {
        if let Some(token) = self.tokens.next() {
            match token {
                Token::Number(value) => Ok(Ast::Number(value.clone())),
                Token::LeftParen => self.paren(),
                _ => Err(format!("Unexpected token: {:?}", token)),
            }
        } else {
            Err("Unexpected end of tokens".to_string())
        }
    }

    fn paren(&mut self) -> Result<Ast, String> {
        let mut depth = 1;
        let mut inner_tokens = Vec::new();

        while let Some(&next_token) = self.tokens.peek() {
            self.tokens.next();

            match next_token {
                Token::LeftParen => {
                    depth += 1;
                    inner_tokens.push(next_token.clone());
                }
                Token::RightParen => {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                    inner_tokens.push(next_token.clone());
                }
                _ => inner_tokens.push(next_token.clone()),
            }
        }

        if depth != 0 {
            return Err("Unclosed parenthesis".into());
        }

        let mut inner_parser = Parser::new(&inner_tokens);
        inner_parser.parse()
    }

    fn infix(&mut self, left: Ast, token: &Token) -> Result<Ast, String> {
        let precedence = Self::get_precedence(token);
        let right = self.expression(precedence + 1)?;

        match token {
            Token::Plus => Ok(Ast::Plus(Box::new(left), Box::new(right))),
            Token::Minus => Ok(Ast::Minus(Box::new(left), Box::new(right))),
            Token::Multiply => Ok(Ast::Multiply(Box::new(left), Box::new(right))),
            Token::Divide => Ok(Ast::Divide(Box::new(left), Box::new(right))),
            _ => Err(format!("Unexpected infix token: {:?}", token)),
        }
    }

    fn get_precedence(token: &Token) -> u8 {
        match token {
            Token::Plus | Token::Minus => 1,
            Token::Multiply | Token::Divide => 2,
            _ => 0,
        }
    }
}
