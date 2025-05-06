use super::types::{Expr, Operator, Token};
use std::{f64::consts, iter::Peekable, slice::Iter};

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
            Some(Token::Num(n)) => self.num(*n),
            Some(Token::LeftParen) => Ok(self.paren()?),
            Some(Token::Minus) => match self.tokens.next() {
                Some(Token::Num(n)) => Ok(Expr::Unary(Operator::Subtraction, Box::new(self.num(*n)?))),
                Some(Token::LeftParen) => Ok(Expr::Unary(Operator::Subtraction, Box::new(self.paren()?))),
                Some(Token::Identifier(id)) => Ok(Expr::Unary(Operator::Subtraction, Box::new(self.ident(id)?))),
                Some(token) => Err(format!("Unexpected token '{}' after unary '-': Expected a number, an opening parenthesis '(', or a valid unary expression.", token)),
                None => Err("Unexpected end of expression: Expected a number, '(', or unary operator before end.".into()),
            },
            Some(Token::Identifier(id)) => self.ident(id),
            Some(token) => Err(format!(
                "Unexpected token '{}' encountered: Expected a number, an opening parenthesis '(', or a unary operator.",
                token
            )),
            None => Err("Unexpected end of expression: Expected a number, '(', or unary operator before end.".into()),
        }
    }

    fn ident(&mut self, id: &str) -> Result<Expr, String> {
        match id {
            "e" => self.num(consts::E),
            "pi" => self.num(consts::PI),
            "phi" => self.num((1.0 + 5.0_f64.sqrt()) / 2.0),
            "sqrt" => self.func(id),
            "ln" => self.func(id),
            "root" => self.func(id),
            "log" => self.func(id),
            "cbrt" => self.func(id),

            "sin" => self.func(id),
            "cos" => self.func(id),
            "tan" => self.func(id),
            "cot" => self.func(id),
            "sec" => self.func(id),
            "csc" => self.func(id),

            "asin" => self.func(id),
            "acos" => self.func(id),
            "atan" => self.func(id),
            "acot" => self.func(id),
            "asec" => self.func(id),
            "acsc" => self.func(id),

            "sinh" => self.func(id),
            "cosh" => self.func(id),
            "tanh" => self.func(id),
            "coth" => self.func(id),
            "sech" => self.func(id),
            "csch" => self.func(id),

            "asinh" => self.func(id),
            "acosh" => self.func(id),
            "atanh" => self.func(id),
            "acoth" => self.func(id),
            "asech" => self.func(id),
            "acsch" => self.func(id),
            _ => Err(format!(
                "Unknown identifier '{}' encountered: Expected a valid identifier.",
                id
            )),
        }
    }

    fn func(&mut self, id: &str) -> Result<Expr, String> {
        match self.tokens.next() {
	    Some(Token::LeftParen) => {
		match id {
		    "root" => {
			let mut radicand = Vec::new();
			while let Some(next_token) = self.tokens.next() {
			    if next_token == &Token::Comma {
				break;
			    }

			    radicand.push(next_token.to_owned());
			}

			Ok(Expr::Function(
			    id.to_string(),
			    vec![Box::new(Parser::new(&radicand).parse()?), Box::new(self.paren()?)]
		        ))
		    }
		    "log" => Ok(Expr::Function(id.to_string(), vec![Box::new(Expr::Num(10.0)), Box::new(self.paren()?)])),
		    _ => Ok(Expr::Function(id.to_string(), vec![Box::new(self.paren()?)]))
		}
	    }
	    Some(Token::Underscore) => {
		let mut base = Vec::new();
		while let Some(next_token) = self.tokens.next() {
		    if next_token == &Token::LeftParen {
			break;
		    }

		    base.push(next_token.to_owned());
		}

                Ok(Expr::Function(id.to_string(), vec![Box::new(Parser::new(&base).parse()?), Box::new(self.paren()?)]))
	    }
	    None => Err("Unexpected end of expression: Expected a number, '(', or unary operator before end".into()),
	    token => Err(format!("Unexpected '{}': Expected parenthesis after '{}'", token.unwrap(), id)),
	}
    }

    fn num(&mut self, num: f64) -> Result<Expr, String> {
        match self.tokens.peek() {
            Some(Token::LeftParen) => {
                self.tokens.next();
                Ok(Expr::Binary(
                    Box::new(Expr::Num(num)),
                    Operator::Multiplication,
                    Box::new(self.paren()?),
                ))
            }
            _ => Ok(Expr::Num(num)),
        }
    }

    fn paren(&mut self) -> Result<Expr, String> {
        let mut tokens = Vec::new();
        let mut depth = 1;

        while let Some(token) = self.tokens.next() {
            match token {
                Token::LeftParen => {
                    depth += 1;
                    tokens.push(token.to_owned());
                }
                Token::RightParen => {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                    tokens.push(token.to_owned());
                }
                _ => tokens.push(token.to_owned()),
            }
        }

        if depth != 0 {
            return Err(format!(
                "Unclosed parenthesis: {} unmatched '('. Expected {} closing ')' before end of expression.",
                depth, depth
            ));
        }

        Parser::new(tokens.as_slice()).parse()
    }

    fn infix(&mut self, left: Expr) -> Result<Expr, String> {
        let token = self.tokens.next().unwrap();
        match token {
            Token::Plus => {
                let right = self.primary(token.precedence() + 1)?;
                Ok(Expr::Binary(
                    Box::new(left),
                    Operator::Addition,
                    Box::new(right),
                ))
            }
            Token::Minus => {
                let right = self.primary(token.precedence() + 1)?;
                Ok(Expr::Binary(
                    Box::new(left),
                    Operator::Subtraction,
                    Box::new(right),
                ))
            }
            Token::Star => {
                let right = self.primary(token.precedence() + 1)?;
                Ok(Expr::Binary(
                    Box::new(left),
                    Operator::Multiplication,
                    Box::new(right),
                ))
            }
            Token::Slash => {
                let right = self.primary(token.precedence() + 1)?;
                Ok(Expr::Binary(
                    Box::new(left),
                    Operator::Division,
                    Box::new(right),
                ))
            }
            Token::Carrot => {
                let right = self.primary(token.precedence())?;
                Ok(Expr::Binary(
                    Box::new(left),
                    Operator::Exponent,
                    Box::new(right),
                ))
            }
            Token::Exclamation => {
                let mut amount: i8 = 1;
                while let Some(token) = self.tokens.peek() {
                    match token {
                        Token::Exclamation => {
                            self.tokens.next();
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
                Box::new(self.paren()?),
            )),
            token => Err(format!(
                "Unknown operator '{}': Expected one of: '+', '-', '*', '/', '^', etc.",
                token
            )),
        }
    }
}
