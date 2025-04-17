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
                c if c.is_digit(10) || c == '.' => {
                    let mut number = String::new();
                    while let Some(c) = self.current_char {
                        if c.is_digit(10) || c == '.' {
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
                    if self.check_for("root") {
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
                'l' => {
                    let mut ident = String::new();
                    for _i in 0..3 {
                        if self.current_char.is_none() {
                            break;
                        }
                        c = self.current_char.unwrap();
                        if c.is_alphabetic() {
                            ident.push(c);
                            self.advance();
                        } else {
                            break;
                        }
                    }

                    if ident == "log" {
                        tokens.push(Token::Log);
                    } else if ident == "ln" {
                        tokens.push(Token::Ln);
                        if self.current_char == Some('(') {
                            tokens.push(Token::LeftParen);
                            self.advance();
                        }
                    }
                }
                's' => {
                    let mut ident = String::new();
                    for _i in 0..4 {
                        if self.current_char.is_none() {
                            break;
                        }
                        c = self.current_char.unwrap();
                        if c.is_alphabetic() {
                            ident.push(c);
                            self.advance();
                        } else {
                            break;
                        }
                    }

                    if ident == "sqrt" {
                        tokens.push(Token::Sqrt);
                    } else if ident == "sin" {
                        tokens.push(Token::Sin);
                        if self.current_char == Some('(') {
                            tokens.push(Token::LeftParen);
                            self.advance();
                        }
                    }
                }
                'c' => {
                    if self.check_for("cos") {
                        tokens.push(Token::Cos);
                    }
                }
                't' => {
                    if self.check_for("tan") {
                        tokens.push(Token::Tan);
                    }
                }
                '!' => {
                    tokens.push(Token::Factorial);
                    self.advance();
                }
                ',' => {
                    tokens.push(Token::Comma);
                    self.advance();
                }
                '_' => {
                    tokens.push(Token::Underscore);
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
                    return Err("Unknown symbol".into());
                }
            }
        }
        Ok(tokens)
    }

    fn check_for(&mut self, name: &str) -> bool {
        let mut c;
        let mut ident = String::new();
        for _i in 0..name.len() {
            if self.current_char.is_none() {
                break;
            }
            c = self.current_char.unwrap();
            if c.is_alphabetic() {
                ident.push(c);
                self.advance();
            } else {
                break;
            }
        }

        return ident == name;
    }
}
