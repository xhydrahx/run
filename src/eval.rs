use types::Token;

mod lexer;
mod types;
mod rpn;

fn evaluate_rpn(rpn: Vec<Token>) -> f64 {
    let mut stack = Vec::new();

    for token in rpn {
        match token {
            Token::Number(value) => stack.push(value),
            Token::Operator(op) => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();

                let result = match op {
                    '+' => a + b,
                    '-' => a - b,
                    '*' => a * b,
                    '/' => a / b,
                    '^' => a.powf(b),
                    _ => panic!("Unknown operator"),
                };

                stack.push(result);
            }
            _ => panic!("Unexpected token in evaluation"),
        }
    }

    stack.pop().unwrap()
}

pub fn run<'a>(input: &str) {
    let tokens = lexer::lex(input);
    let rpn = rpn::into(tokens.clone());
    println!("{}", evaluate_rpn(rpn));
}
