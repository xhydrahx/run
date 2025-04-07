use super::types::Token;
use std::str::Chars;

pub struct Lexer<'a> {
    input: Chars<'a>,
    current_char: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input: input.chars(),
            current_char: None,
        };
        lexer.advance();
        lexer
    }

    fn advance(&mut self) {
        self.current_char = self.input.next();
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(c) = self.current_char {
            match c {
                ' ' | '\n' | '\t' => self.advance(),
                c if c.is_digit(10) => {
                    tokens.push(Token::Number(c.to_digit(10).unwrap() as f64));
                    self.advance();
                }
                '+' => {
                    tokens.push(Token::Addition);
                    self.advance();
                }
                '-' => {
                    tokens.push(Token::Subtraction);
                    self.advance();
                }
                '*' => {
                    tokens.push(Token::Multiplication);
                    self.advance();
                }
                '/' => {
                    tokens.push(Token::Division);
                    self.advance();
                }
                '^' => {
                    tokens.push(Token::Exponent);
                    self.advance();
                }
                '(' => {
                    tokens.push(Token::LeftParen);
                    self.advance();
                }
                ')' => {
                    tokens.push(Token::RightParen);
                    self.advance();
                }
                _ => self.advance(),
            }
        }
        tokens
    }
}
