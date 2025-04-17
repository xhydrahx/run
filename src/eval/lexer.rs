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
        while let Some(c) = self.current_char {
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
                    if self.check_for(&["root"]).is_some() {
                        tokens.push(Token::Root);
                    } else {
                        return Err("Unknown identifier starting with 'r'".into());
                    }
                }
                'e' => {
                    tokens.push(Token::E);
                    self.advance();
                }
                'p' => {
                    if let Some(keyword) = self.check_for(&["pi", "phi"]) {
                        match keyword.as_str() {
                            "pi" => tokens.push(Token::Pi),
                            "phi" => tokens.push(Token::Phi),
                            _ => unreachable!(),
                        }
                    } else {
                        return Err("Unknown identifier starting with 'p'".into());
                    }
                }
                'l' => {
                    if let Some(keyword) = self.check_for(&["ln", "log"]) {
                        match keyword.as_str() {
                            "ln" => {
                                tokens.push(Token::Ln);
                                if self.current_char == Some('(') {
                                    tokens.push(Token::LeftParen);
                                    self.advance();
                                }
                            }
                            "log" => tokens.push(Token::Log),
                            _ => unreachable!(),
                        }
                    } else {
                        return Err("Unknown identifier starting with 'l'".into());
                    }
                }
                's' => {
                    if let Some(keyword) = self.check_for(&["sin", "sqrt"]) {
                        match keyword.as_str() {
                            "sin" => {
                                tokens.push(Token::Sin);
                                if self.current_char == Some('(') {
                                    tokens.push(Token::LeftParen);
                                    self.advance();
                                }
                            }
                            "sqrt" => tokens.push(Token::Sqrt),
                            _ => unreachable!(),
                        }
                    } else {
                        return Err("Unknown identifier starting with 'l'".into());
                    }
                }
                'c' => {
                    if self.check_for(&["cos"]).is_some() {
                        tokens.push(Token::Cos);
                    } else {
                        return Err("Unknown identifier starting with 'c'".into());
                    }
                }
                't' => {
                    if self.check_for(&["tan"]).is_some() {
                        tokens.push(Token::Tan);
                    } else {
                        return Err("Unknown identifier starting with 't'".into());
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

    fn check_for(&mut self, keywords: &[&str]) -> Option<String> {
        let saved_input = self.input.clone();
        let saved_char = self.current_char;

        let mut ident = String::new();
        while let Some(c) = self.current_char {
            if c.is_alphabetic() {
                ident.push(c);
                self.advance();
            } else {
                break;
            }
        }

        for &keyword in keywords {
            if ident == keyword {
                return Some(ident);
            }
        }

        self.input = saved_input;
        self.current_char = saved_char;
        None
    }
}
