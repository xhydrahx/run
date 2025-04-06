#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(f64),

    Plus,
    Minus,
    Multiply,
    Divide,

    LeftParen,
    RightParen,
}

#[derive(Debug)]
pub enum Ast {
    Number(f64),

    Plus(Box<Ast>, Box<Ast>),
    Minus(Box<Ast>, Box<Ast>),
    Multiply(Box<Ast>, Box<Ast>),
    Divide(Box<Ast>, Box<Ast>),
}
