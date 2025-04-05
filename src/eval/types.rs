#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    Number(f64),
    Operator(char),
    Parenthesis(char),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Associativity {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Clone)]
pub struct OperatorInfo {
    pub precedence: u8,
    pub associativity: Associativity,
}

pub fn get_operator_info(op: char) -> Option<OperatorInfo> {
    match op {
        '+' | '-' => Some(OperatorInfo {
            precedence: 2,
            associativity: Associativity::Left,
        }),
        '*' | '/' => Some(OperatorInfo {
            precedence: 3,
            associativity: Associativity::Left,
        }),
        '^' => Some(OperatorInfo {
            precedence: 4,
            associativity: Associativity::Right,
        }),
        _ => None,
    }
}
