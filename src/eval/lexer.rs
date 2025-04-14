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

    pub fn lex(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        while let Some(mut c) = self.current_char {
            match c {
                ' ' | '\n' | '\t' => self.advance(),
                '0'..'9' | '.' => {
                    let mut number = String::new();
                    while let Some(c) = self.current_char {
                        if c.is_digit(10) {
                            number.push(c);
                        } else if c == '.' {
                            number.push(c);
                        } else {
                            break;
                        }
                        self.advance();
                    }
                    tokens.push(Token::Number(
                        number
                            .parse::<f64>()
                            .expect("Failed to parse a string into a number"),
                    ));
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
                'r' => {
                    let mut ident = String::new();
                    for _i in 0..4 {
                        c = self.current_char.unwrap();
                        if c.is_alphabetic() {
                            ident.push(c);
                            self.advance();
                        } else {
                            return Err("Unknown Symbol".into());
                        }
                    }

                    if ident == "root" {
                        tokens.push(Token::Root);
                    }
                }
                'e' => {
                    tokens.push(Token::E);
                    self.advance();
                }
                'p' => {
                    self.advance();
                    if self.current_char == Some('i') {
                        tokens.push(Token::Pi);
                        self.advance();
                    } else if self.current_char == Some('h') {
                        self.advance();
                        if self.current_char == Some('i') {
                            tokens.push(Token::Phi);
                            self.advance();
                        }
                    }
                }
                ',' => {
                    tokens.push(Token::Comma);
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
                _ => {
                    return Err("Unknown Symbol".into());
                }
            }
        }
        Ok(tokens)
    }
}
