#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(f64),

    Addition,
    Subtraction,
    Multiplication,
    Division,

    LeftParen,
    RightParen,
}

#[derive(Debug)]
pub enum Ast {
    Number(f64),

    Addition(Box<Ast>, Box<Ast>),
    Subtraction(Box<Ast>, Box<Ast>),
    Multiplication(Box<Ast>, Box<Ast>),
    Division(Box<Ast>, Box<Ast>),
}
