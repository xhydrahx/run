use std::{iter::Peekable, slice::Iter};

use crate::eval::{types::{Expr, Token}, parser};

pub fn paren(tokens: &mut Peekable<Iter<Token>>) -> Result<Expr, String> {
    let mut inside = Vec::new();
    let mut depth = 1;

    while let Some(token) = tokens.next() {
        match token {
            Token::LeftParen => {
                depth += 1;
                inside.push(token.to_owned());
            }
            Token::RightParen => {
                depth -= 1;
                if depth == 0 {
                    break;
                }
                inside.push(token.to_owned());
            }
            _ => inside.push(token.to_owned()),
        }
    }

    if depth != 0 {
        return Err(format!(
            "Unclosed parenthesis: {} unmatched '('. Expected {} closing ')' before end of expression.",
            depth, depth
        ));
    }

    parser::parse(inside)
}
