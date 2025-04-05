use std::collections::VecDeque;
use super::types::{get_operator_info, Associativity, Token};

pub fn into(tokens: Vec<Token>) -> Vec<Token> {
    let mut output = Vec::new();
    let mut operators = VecDeque::new();

    for token in tokens {
        match token {
            Token::Number(_) => output.push(token),
            Token::Operator(op) => {
                while let Some(&top) = operators.front() {
                    if let Token::Operator(top_op) = top {
                        if let Some(op_info) = get_operator_info(op) {
                            if let Some(top_op_info) = get_operator_info(top_op) {
                                if (op_info.associativity == Associativity::Left
                                    && op_info.precedence <= top_op_info.precedence)
                                    || (op_info.associativity == Associativity::Right
                                        && op_info.precedence < top_op_info.precedence)
                                {
                                    output.push(operators.pop_front().unwrap());
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                }
                operators.push_front(Token::Operator(op));
            }
            Token::Parenthesis(ch) => {
                match ch {
                    '(' => operators.push_front(token),
                    ')' => {
                        while let Some(top) = operators.pop_front() {
                            if let Token::Parenthesis('(') = top {
                                break;
                            } else {
                                output.push(top);
                            }
                        }
                    }
                    _ => {
                        panic!("Unknown Parenthesis Char: {}", ch);
                    }
                }
            }
        }
    }

    while let Some(op) = operators.pop_front() {
        output.push(op);
    }

    output
}
