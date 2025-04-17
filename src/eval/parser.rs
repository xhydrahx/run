use super::types::{Ast, Token};
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

    pub fn parse(&mut self) -> Result<Ast, String> {
        self.expression(0)
    }

    fn expression(&mut self, precedence: u8) -> Result<Ast, String> {
        let mut left = self.primary()?;

        while let Some(&token) = self.tokens.peek() {
            let token_precedence = Self::precedence(token);
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
            if let Ok(ast) = self.functions(token) {
                Ok(ast)
            } else {
                match token {
                    Token::Number(value) => Ok(Ast::Number(value.clone())),
                    Token::E => Ok(Ast::Number(consts::E)),
                    Token::Pi => Ok(Ast::Number(consts::PI)),
                    Token::Phi => Ok(Ast::Number((1.0 + 5.0_f64.sqrt()) / 2.0)),
                    Token::LeftParen => Ok(self.paren()?),
                    Token::Subtraction => match self.tokens.next() {
                        Some(Token::Number(value)) => Ok(Ast::Number(-value.clone())),
                        Some(Token::LeftParen) => {
                            let mut number = Vec::new();
                            let mut depth = 1;
                            while let Some(next_token) = self.tokens.next() {
                                if next_token == &Token::LeftParen {
                                    depth += 1;
                                }
                                if next_token == &Token::RightParen {
                                    depth -= 1;
                                    if depth == 0 {
                                        break;
                                    }
                                }
                                number.push(next_token.clone());
                            }
                            Ok(Parser::new(&number).parse()?)
                        }
                        Some(Token::E) => Ok(Ast::Number(-consts::E)),
                        Some(Token::Pi) => Ok(Ast::Number(-consts::PI)),
                        Some(Token::Phi) => Ok(Ast::Number(-((1.0 + 5.0_f64.sqrt()) / 2.0))),
                        _ => Err("Unexpected symbol".into()),
                    },
                    _ => Err("Unexpected symbol".into()),
                }
            }
        } else {
            Err("Unexpected end of expression".into())
        }
    }

    fn functions(&mut self, token: &Token) -> Result<Ast, String> {
        match token {
            Token::Root => match self.tokens.next() {
                Some(Token::LeftParen) => {
                    let mut radicand = Vec::new();
                    let mut depth = 1;
                    while let Some(next_token) = self.tokens.next() {
                        if next_token == &Token::LeftParen {
                            depth += 1;
                        }

                        if next_token == &Token::RightParen {
                            depth -= 1;
                            if depth == 0 {
                                return Ok(Ast::Root(
                                    Box::new(Parser::new(&radicand).parse()?),
                                    Box::new(Ast::Number(2.0)),
                                ));
                            }
                        }

                        if next_token == &Token::Comma {
                            break;
                        }

                        radicand.push(next_token.clone());
                    }

                    let mut index = Vec::new();
                    let mut depth = 1;
                    while let Some(next_token) = self.tokens.next() {
                        if next_token == &Token::LeftParen {
                            depth += 1;
                        }
                        if next_token == &Token::RightParen {
                            depth -= 1;
                            if depth == 0 {
                                break;
                            }
                        }

                        index.push(next_token.clone());
                    }

                    Ok(Ast::Root(
                        Box::new(Parser::new(&radicand).parse()?),
                        Box::new(Parser::new(&index).parse()?),
                    ))
                }
                _ => Err("Incorrect isage of root function".into()),
            },
            Token::Log => match self.tokens.next() {
                Some(Token::Underscore) => {
                    let mut base = Vec::new();
                    while let Some(next_token) = self.tokens.next() {
                        if next_token == &Token::LeftParen {
                            break;
                        }

                        base.push(next_token.clone());
                    }

                    let mut argument = Vec::new();
                    let mut depth = 1;
                    while let Some(next_token) = self.tokens.next() {
                        if next_token == &Token::LeftParen {
                            depth += 1;
                        }
                        if next_token == &Token::RightParen {
                            depth -= 1;
                            if depth == 0 {
                                break;
                            }
                        }

                        argument.push(next_token.clone());
                    }

                    Ok(Ast::Log(
                        Box::new(Parser::new(&base).parse()?),
                        Box::new(Parser::new(&argument).parse()?),
                    ))
                }
                Some(Token::LeftParen) => {
                    let mut argument = Vec::new();
                    let mut depth = 1;
                    while let Some(next_token) = self.tokens.next() {
                        if next_token == &Token::LeftParen {
                            depth += 1;
                        }
                        if next_token == &Token::RightParen {
                            depth -= 1;
                            if depth == 0 {
                                break;
                            }
                        }

                        argument.push(next_token.clone());
                    }

                    Ok(Ast::Log(
                        Box::new(Ast::Number(10.0)),
                        Box::new(Parser::new(&argument).parse()?),
                    ))
                }
                _ => Err("Incorrect usage of log function".into()),
            },
            Token::Ln => match self.tokens.next() {
                Some(Token::LeftParen) => {
                    let mut argument = Vec::new();
                    let mut depth = 1;
                    while let Some(next_token) = self.tokens.next() {
                        if next_token == &Token::LeftParen {
                            depth += 1;
                        }
                        if next_token == &Token::RightParen {
                            depth -= 1;
                            if depth == 0 {
                                break;
                            }
                        }

                        argument.push(next_token.clone());
                    }

                    Ok(Ast::Ln(Box::new(Parser::new(&argument).parse()?)))
                }
                _ => Err("Incorrect usage of ln function".into()),
            },
            Token::Sin => match self.tokens.next() {
                Some(Token::LeftParen) => {
                    let mut argument = Vec::new();
                    let mut depth = 1;
                    while let Some(next_token) = self.tokens.next() {
                        if next_token == &Token::LeftParen {
                            depth += 1;
                        }
                        if next_token == &Token::RightParen {
                            depth -= 1;
                            if depth == 0 {
                                break;
                            }
                        }

                        argument.push(next_token.clone());
                    }

                    Ok(Ast::Sin(Box::new(Parser::new(&argument).parse()?)))
                }
                _ => Err("Incorrect usage of sin function".into()),
            },
            Token::Cos => match self.tokens.next() {
                Some(Token::LeftParen) => {
                    let mut argument = Vec::new();
                    let mut depth = 1;
                    while let Some(next_token) = self.tokens.next() {
                        if next_token == &Token::LeftParen {
                            depth += 1;
                        }
                        if next_token == &Token::RightParen {
                            depth -= 1;
                            if depth == 0 {
                                break;
                            }
                        }

                        argument.push(next_token.clone());
                    }

                    Ok(Ast::Cos(Box::new(Parser::new(&argument).parse()?)))
                }
                _ => Err("Incorrect usage of cos function".into()),
            },
            Token::Tan => match self.tokens.next() {
                Some(Token::LeftParen) => {
                    let mut argument = Vec::new();
                    let mut depth = 1;
                    while let Some(next_token) = self.tokens.next() {
                        if next_token == &Token::LeftParen {
                            depth += 1;
                        }
                        if next_token == &Token::RightParen {
                            depth -= 1;
                            if depth == 0 {
                                break;
                            }
                        }

                        argument.push(next_token.clone());
                    }

                    Ok(Ast::Tan(Box::new(Parser::new(&argument).parse()?)))
                }
                _ => Err("Incorrect usage of tan function".into()),
            },
            _ => Err("Unknown function".into()),
        }
    }

    fn paren(&mut self) -> Result<Ast, String> {
        let mut depth = 1;
        let mut tokens = Vec::new();

        while let Some(token) = self.tokens.next() {
            match token {
                Token::LeftParen => {
                    depth += 1;
                    tokens.push(token.clone());
                }
                Token::RightParen => {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                    tokens.push(token.clone());
                }
                _ => tokens.push(token.clone()),
            }
        }

        if depth != 0 {
            return Err("Unclosed parenthesis".into());
        }

        Parser::new(&tokens).parse()
    }

    fn infix(&mut self, left: Ast, token: &Token) -> Result<Ast, String> {
        if token.is_unary() {
            return match token {
                Token::Factorial => Ok(Ast::Factorial(Box::new(left))),
                _ => Err("Unexpected unary operator".into()),
            };
        }

        let precedence = Self::precedence(token);
        let right = self.expression(precedence + 1)?;

        match token {
            Token::Addition => Ok(Ast::Addition(Box::new(left), Box::new(right))),
            Token::Subtraction => Ok(Ast::Subtraction(Box::new(left), Box::new(right))),
            Token::Multiplication => Ok(Ast::Multiplication(Box::new(left), Box::new(right))),
            Token::Division => Ok(Ast::Division(Box::new(left), Box::new(right))),
            Token::Exponent => Ok(Ast::Exponent(Box::new(left), Box::new(right))),
            _ => Err("Unexpected infix symbol".into()),
        }
    }

    fn precedence(token: &Token) -> u8 {
        match token {
            Token::Addition | Token::Subtraction => 1,
            Token::Multiplication | Token::Division => 2,
            Token::Exponent => 3,
            Token::Factorial => 4,
            _ => 0,
        }
    }
}
