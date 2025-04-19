use super::types::Token;
use std::str::Chars;

pub struct Lexer<'a> {
    expr: Chars<'a>,
    ch: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            expr: input.chars(),
            ch: input.chars().next(),
        }
    }

    fn advance(&mut self) {
        self.ch = self.expr.next();
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        while let Some(c) = self.ch {
            match c {
                '0'..='9' | '.' => tokens.push(self.number()),
                '+' => {
                    tokens.push(Token::Plus);
                    self.advance();
                }
                '-' => {
                    tokens.push(Token::Minus);
                    self.advance();
                }
                '*' => {
                    tokens.push(Token::Star);
                    self.advance();
                }
                '/' => {
                    tokens.push(Token::Slash);
                    self.advance();
                }
                '^' => {
                    tokens.push(Token::Carrot);
                    self.advance();
                }
                ' ' | '\t' | '\n' => self.advance(),
                _ => {
                    return Err(format!("Unknown symbol '{}'", c));
                }
            }
        }
        Ok(tokens)
    }

    fn number(&mut self) -> Token {
        let mut num = String::new();
        while let Some(c) = self.ch {
            if c.is_numeric() || c == '.' {
                num.push(c);
                self.advance();
            } else {
                break;
            }
        }
        Token::Num(
            num.parse::<f64>()
                .expect("Failed to parse a string into a number"),
        )
    }
}
