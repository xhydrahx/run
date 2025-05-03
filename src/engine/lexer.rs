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
            ch: None,
        }
    }

    fn advance(&mut self) {
        self.ch = self.expr.next();
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        self.advance();
	
        while let Some(c) = self.ch {
            match c {
                '0'..='9' | '.' => {
                    let mut num = String::new();
                    while let Some(c) = self.ch {
                        if c.is_digit(10) || c == '.' {
                            num.push(c);
                        } else {
                            break;
                        }
                        self.advance();
                    }
                    tokens.push(Token::Num(
                        num.parse::<f64>()
                            .expect("Failed to parse a string into a number"),
                    ));
                }
                'a'..='z' | 'A'..='Z' => {
                    let mut identifier = String::new();
                    while let Some(c) = self.ch {
                        if c.is_alphabetic() {
                            identifier.push(c);
                        } else {
			    break;
			}
                        self.advance();
                    }
		    
                    tokens.push(Token::Identifier(identifier));
                }
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
                '(' => {
                    tokens.push(Token::LeftParen);
                    self.advance();
                }
                ')' => {
                    tokens.push(Token::RightParen);
                    self.advance();
                }
                '!' => {
                    tokens.push(Token::Exclamation);
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
                ' ' | '\t' | '\n' => self.advance(),
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
