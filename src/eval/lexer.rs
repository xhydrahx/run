use crate::eval::types::Token;
use std::{iter::Peekable, str::Chars};

pub fn lex(expr: &mut Peekable<Chars>) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();

    while let Some(c) = expr.peek() {
        match c {
            '0'..='9' | '.' => {
                let mut num = String::new();
                while let Some(c) = expr.peek() {
                    if c.is_digit(10) || c == &'.' {
                        num.push(*c);
                    } else {
                        break;
                    }
                    expr.next();
                }
                tokens.push(Token::Num(
                    num.parse::<f64>()
                        .expect("Failed to parse a string into a number"),
                ));
            }
            'a'..='z' | 'A'..='Z' => {
                let mut identifier = String::new();
                while let Some(c) = expr.peek() {
                    if c.is_alphabetic() {
                        identifier.push(*c);
                    } else {
                        break;
                    }
                    expr.next();
                }

                tokens.push(Token::Identifier(identifier));
            }
            '+' => {
                tokens.push(Token::Plus);
                expr.next();
            }
            '-' => {
                tokens.push(Token::Minus);
                expr.next();
            }
            '*' => {
                tokens.push(Token::Star);
                expr.next();
            }
            '/' => {
                tokens.push(Token::Slash);
                expr.next();
            }
            '^' => {
                tokens.push(Token::Carrot);
                expr.next();
            }
            '(' => {
                tokens.push(Token::LeftParen);
                expr.next();
            }
            ')' => {
                tokens.push(Token::RightParen);
                expr.next();
            }
            '!' => {
                tokens.push(Token::Exclamation);
                expr.next();
            }
            ',' => {
                tokens.push(Token::Comma);
                expr.next();
            }
            '_' => {
                tokens.push(Token::Underscore);
                expr.next();
            }
            '%' => {
                tokens.push(Token::Percent);
                expr.next();
            }
            '|' => {
                tokens.push(Token::Bar);
                expr.next();
            }
            '=' => {
                tokens.push(Token::Equal);
                expr.next();
            }
            ' ' | '\t' | '\n' => {
                expr.next();
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
