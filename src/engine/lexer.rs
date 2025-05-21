use super::types::Token;
use std::{iter::Peekable, str::Chars};

pub struct Lexer<'a> {
    expr: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            expr: input.chars().peekable(),
        }
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        while let Some(c) = self.expr.peek() {
            match c {
                '0'..='9' | '.' => {
                    let mut num = String::new();
                    while let Some(c) = self.expr.peek() {
                        if c.is_digit(10) || c == &'.' {
                            num.push(*c);
                        } else {
                            break;
                        }
                        self.expr.next();
                    }
                    tokens.push(Token::Num(
                        num.parse::<f64>()
                            .expect("Failed to parse a string into a number"),
                    ));
                }
                'a'..='z' | 'A'..='Z' => {
                    let mut identifier = String::new();
                    while let Some(c) = self.expr.peek() {
                        if c.is_alphabetic() {
                            identifier.push(*c);
                        } else {
                            break;
                        }
                        self.expr.next();
                    }

                    tokens.push(Token::Identifier(identifier));
                }
                '+' => {
                    tokens.push(Token::Plus);
                    self.expr.next();
                }
                '-' => {
                    tokens.push(Token::Minus);
                    self.expr.next();
                }
                '*' => {
                    tokens.push(Token::Star);
                    self.expr.next();
                }
                '/' => {
                    tokens.push(Token::Slash);
                    self.expr.next();
                }
                '^' => {
                    tokens.push(Token::Carrot);
                    self.expr.next();
                }
                '(' => {
                    tokens.push(Token::LeftParen);
                    self.expr.next();
                }
                ')' => {
                    tokens.push(Token::RightParen);
                    self.expr.next();
                }
                '!' => {
                    tokens.push(Token::Exclamation);
                    self.expr.next();
                }
                ',' => {
                    tokens.push(Token::Comma);
                    self.expr.next();
                }
                '_' => {
                    tokens.push(Token::Underscore);
                    self.expr.next();
                }
                '%' => {
                    tokens.push(Token::Percent);
                    self.expr.next();
                }
                '|' => {
                    tokens.push(Token::Bar);
                    self.expr.next();
                }
                ' ' | '\t' | '\n' => {
                    self.expr.next();
                }
                _ => {
                    return Err(format!(
                        "Unknown token '{}': This token is not recognized as part of a valid expression. Check for typos or invalid characters.",
                        c
                    ));
                }
            }
        }
        Ok(tokens)
    }
}
