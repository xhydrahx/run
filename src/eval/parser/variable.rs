use std::{iter::Peekable, slice::Iter};

use crate::eval::{environment, parser, types::{Expr, Token}};

pub fn parse(tokens: &mut Peekable<Iter<Token>>, id: &str) -> Result<Expr, String> {
    {
        let variables = environment::fetch_variables().lock().unwrap();
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
    let mut variables = environment::fetch_variables().lock().unwrap();
    variables.push(Expr::Variable(id.to_string(), Box::new(expr)));
    return Ok(Expr::Num(1.0));
}
