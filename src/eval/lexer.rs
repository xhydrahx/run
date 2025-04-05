use super::types::Token;

pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    
    while let Some(ch) = chars.next() {
        match ch {
            '+' | '-' | '*' | '/' | '^' => tokens.push(Token::Operator(ch)),
            '(' | ')' => tokens.push(Token::Parenthesis(ch)),  // Only match '(' and ')'
            '0'..='9' | '.' => {
                let mut number = ch.to_string();
                while let Some(&next_ch) = chars.peek() {
                    if next_ch.is_digit(10) || next_ch == '.' {
                        number.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Number(number.parse().unwrap()));
            }
            _ if ch.is_whitespace() => continue,
            _ => panic!("Invalid character found in input."),
        }
    }
    
    tokens
}
