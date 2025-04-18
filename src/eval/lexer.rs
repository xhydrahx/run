use super::types::{Token, TrigType};
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
                    if let Some(keyword) = self.check_for(&["sin", "sec", "sinh", "sech", "sqrt"]) {
                        match keyword.as_str() {
                            "sin" => {
                                tokens.push(Token::Trig(TrigType::Sin));
                                if self.current_char == Some('(') {
                                    tokens.push(Token::LeftParen);
                                    self.advance();
                                }
                            }
                            "sec" => {
                                tokens.push(Token::Trig(TrigType::Sec));
                                if self.current_char == Some('(') {
                                    tokens.push(Token::LeftParen);
                                    self.advance();
                                }
                            }
                            "sinh" => tokens.push(Token::Trig(TrigType::Sinh)),
                            "sech" => tokens.push(Token::Trig(TrigType::Sech)),
                            "sqrt" => tokens.push(Token::Sqrt),
                            _ => unreachable!(),
                        }
                    } else {
                        return Err("Unknown identifier starting with 'l'".into());
                    }
                }
                'c' => {
                    if let Some(keyword) =
                        self.check_for(&["cos", "csc", "cot", "cosh", "csch", "coth"])
                    {
                        match keyword.as_str() {
                            "cos" => {
                                tokens.push(Token::Trig(TrigType::Cos));
                                if self.current_char == Some('(') {
                                    tokens.push(Token::LeftParen);
                                    self.advance();
                                }
                            }
                            "csc" => {
                                tokens.push(Token::Trig(TrigType::Csc));
                                if self.current_char == Some('(') {
                                    tokens.push(Token::LeftParen);
                                    self.advance();
                                }
                            }
                            "cot" => {
                                tokens.push(Token::Trig(TrigType::Cot));
                                if self.current_char == Some('(') {
                                    tokens.push(Token::LeftParen);
                                    self.advance();
                                }
                            }
                            "cosh" => tokens.push(Token::Trig(TrigType::Cosh)),
                            "csch" => tokens.push(Token::Trig(TrigType::Csch)),
                            "coth" => tokens.push(Token::Trig(TrigType::Coth)),
                            _ => unreachable!(),
                        }
                    } else {
                        return Err("Unknown identifier starting with 'c'".into());
                    }
                }
                't' => {
                    if let Some(keyword) = self.check_for(&["tan", "tanh"]) {
                        match keyword.as_str() {
                            "tan" => {
                                tokens.push(Token::Trig(TrigType::Tan));
                                if self.current_char == Some('(') {
                                    tokens.push(Token::LeftParen);
                                    self.advance();
                                }
                            }
                            "tanh" => tokens.push(Token::Trig(TrigType::Tanh)),
                            _ => unreachable!(),
                        }
                    } else {
                        return Err("Unknown identifier starting with 't'".into());
                    }
                }
                'a' => {
                    if let Some(keyword) = self.check_for(&[
                        "asin", "acos", "atan", "acsc", "asec", "acot", "asinh", "acosh", "atanh",
                        "acsch", "asech", "acoth",
                    ]) {
                        match keyword.as_str() {
                            "asin" => {
                                tokens.push(Token::Trig(TrigType::Arcsin));
                                if self.current_char == Some('(') {
                                    tokens.push(Token::LeftParen);
                                    self.advance();
                                }
                            }
                            "acos" => {
                                tokens.push(Token::Trig(TrigType::Arccos));
                                if self.current_char == Some('(') {
                                    tokens.push(Token::LeftParen);
                                    self.advance();
                                }
                            }
                            "atan" => {
                                tokens.push(Token::Trig(TrigType::Arctan));
                                if self.current_char == Some('(') {
                                    tokens.push(Token::LeftParen);
                                    self.advance();
                                }
                            }
                            "acot" => {
                                tokens.push(Token::Trig(TrigType::Arccot));
                                if self.current_char == Some('(') {
                                    tokens.push(Token::LeftParen);
                                    self.advance();
                                }
                            }
                            "asec" => {
                                tokens.push(Token::Trig(TrigType::Arcsec));
                                if self.current_char == Some('(') {
                                    tokens.push(Token::LeftParen);
                                    self.advance();
                                }
                            }
                            "acsc" => {
                                tokens.push(Token::Trig(TrigType::Arccsc));
                                if self.current_char == Some('(') {
                                    tokens.push(Token::LeftParen);
                                    self.advance();
                                }
                            }
                            "asinh" => tokens.push(Token::Trig(TrigType::Arcsinh)),
                            "acosh" => tokens.push(Token::Trig(TrigType::Arccosh)),
                            "atanh" => tokens.push(Token::Trig(TrigType::Arctanh)),
                            "acoth" => tokens.push(Token::Trig(TrigType::Arccoth)),
                            "asech" => tokens.push(Token::Trig(TrigType::Arcsech)),
                            "acsch" => tokens.push(Token::Trig(TrigType::Arccsch)),
                            _ => unreachable!(),
                        }
                    } else {
                        return Err("Unknown identifier starting with 'a'".into());
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
