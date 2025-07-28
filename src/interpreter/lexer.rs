use crate::interpreter::error::CalcError;

#[derive(Debug, PartialEq)]
pub enum Token {
    Atomic(f64),

    Plus,
    Minus,
    Star,
    Slash,
}

pub fn lex(input: String) -> Result<Vec<Token>, CalcError> {
    let mut source = input.chars().peekable();
    let mut tokens = Vec::new();

    while let Some(ch) = source.peek() {
        match ch {
            '0'..'9' | '.' => {
                let mut a = String::new();

                while let Some(c) = source.peek() {
                    if !c.is_numeric() && c != &'.' {
                        break;
                    }
                    a.push(*c);
                    source.next();
                }

                tokens.push(Token::Atomic(a.parse::<f64>()?));
            }
            '+' => {
                tokens.push(Token::Plus);
                source.next();
            }
            '-' => {
                tokens.push(Token::Minus);
                source.next();
            }
            '*' => {
                tokens.push(Token::Star);
                source.next();
            }
            '/' => {
                tokens.push(Token::Slash);
                source.next();
            }
            ' ' | '\t' | '\n' => {
                source.next();
            }
            _ => {
                return Err(CalcError::Unknown(format!("Char '{}'", ch)));
            }
        }
    }

    Ok(tokens)
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_number() {
        let tokens = lex("2".into()).expect("Failed to lex");
        let expected_tokens = vec![Token::Atomic(2.0)];

        assert_eq!(tokens.len(), expected_tokens.len());
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_decimal() {
        let tokens = lex("3.14154".into()).expect("Failed to lex");
        let expected_tokens = vec![Token::Atomic(3.14154)];

        assert_eq!(tokens.len(), expected_tokens.len());
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_lex() {
        let tokens = lex("19+10".into()).expect("Failed to lex");
        let expected_tokens = vec![Token::Atomic(19.0), Token::Plus, Token::Atomic(10.0)];

        assert_eq!(tokens.len(), expected_tokens.len());
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_all_arithmetic() {
        let tokens = lex("19+10*10-3/69".into()).expect("Failed to lex");
        let expected_tokens = vec![
            Token::Atomic(19.0),
            Token::Plus,
            Token::Atomic(10.0),
            Token::Star,
            Token::Atomic(10.0),
            Token::Minus,
            Token::Atomic(3.0),
            Token::Slash,
            Token::Atomic(69.0),
        ];

        assert_eq!(tokens.len(), expected_tokens.len());
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_unknown_token() {
        match lex("2@2".into()) {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }
    }

    #[test]
    fn test_nothing() {
        assert!(lex("".into()).expect("Failed to lex").is_empty());
    }
}
