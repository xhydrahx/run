use std::{iter::Peekable, slice::Iter};

use crate::eval::{
    parser::{self, group},
    types::{Expr, Token},
    variables,
};

pub fn ident(tokens: &mut Peekable<Iter<Token>>, id: &str) -> Result<Expr, String> {
    match id {
        "sqrt" => func(tokens, id),
        "ln" => func(tokens, id),
        "root" => func(tokens, id),
        "log" => func(tokens, id),
        "cbrt" => func(tokens, id),

        "sin" => func(tokens, id),
        "cos" => func(tokens, id),
        "tan" => func(tokens, id),
        "cot" => func(tokens, id),
        "sec" => func(tokens, id),
        "csc" => func(tokens, id),

        "asin" => func(tokens, id),
        "acos" => func(tokens, id),
        "atan" => func(tokens, id),
        "acot" => func(tokens, id),
        "asec" => func(tokens, id),
        "acsc" => func(tokens, id),

        "sinh" => func(tokens, id),
        "cosh" => func(tokens, id),
        "tanh" => func(tokens, id),
        "coth" => func(tokens, id),
        "sech" => func(tokens, id),
        "csch" => func(tokens, id),

        "asinh" => func(tokens, id),
        "acosh" => func(tokens, id),
        "atanh" => func(tokens, id),
        "acoth" => func(tokens, id),
        "asech" => func(tokens, id),
        "acsch" => func(tokens, id),

        _ => {
            {
                let variables = variables::get_variables().lock().unwrap();
                for expr in variables.iter() {
                    if let Expr::Variable(ident, value) = expr {
                        if ident.as_str() == id {
                            return Ok(Expr::Variable(ident.to_string(), value.to_owned()));
                        }
                    }
                }

                if tokens.peek() != Some(&&Token::Equal) {
                    return Err(format!(
                        "Unknown variable '{}': Expected a valid variable that has been defined",
                        id
                    ));
                }
            }

            tokens.next();
            let expr = parser::primary(tokens, 0)?;
            let mut variables = variables::get_variables().lock().unwrap();
            variables.push(Expr::Variable(id.to_string(), Box::new(expr)));
            return Ok(Expr::Num(1.0));
        }
    }
}

fn func(tokens: &mut Peekable<Iter<Token>>, id: &str) -> Result<Expr, String> {
    match tokens.next() {
        Some(Token::LeftParen) => match id {
            "root" => {
                let mut radicand = Vec::new();
                while let Some(next_token) = tokens.next() {
                    if next_token == &Token::Comma {
                        break;
                    }

                    radicand.push(next_token.to_owned());
                }
                Ok(Expr::Function(
                    id.to_string(),
                    vec![
                        Box::new(parser::parse(radicand)?),
                        Box::new(group::paren(tokens)?),
                    ],
                ))
            }
            "log" => Ok(Expr::Function(
                id.to_string(),
                vec![Box::new(Expr::Num(10.0)), Box::new(group::paren(tokens)?)],
            )),
            _ => Ok(Expr::Function(
                id.to_string(),
                vec![Box::new(group::paren(tokens)?)],
            )),
        },
        Some(Token::Underscore) => {
            let mut base = Vec::new();
            while let Some(next_token) = tokens.next() {
                if next_token == &Token::LeftParen {
                    break;
                }

                base.push(next_token.to_owned());
            }

            Ok(Expr::Function(
                id.to_string(),
                vec![
                    Box::new(parser::parse(base)?),
                    Box::new(group::paren(tokens)?),
                ],
            ))
        }
        None => Err(
            "Unexpected end of expression: Expected a number, '(', or unary operator before end"
                .into(),
        ),
        token => Err(format!(
            "Unexpected '{}': Expected parenthesis after '{}'",
            token.unwrap(),
            id
        )),
    }
}
