#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(f64),

    Addition,
    Subtraction,
    Multiplication,
    Division,

    Exponent,
    Root,

    LeftParen,
    RightParen,
    Comma,
}

#[derive(Debug)]
pub enum Ast {
    Number(f64),

    Addition(Box<Ast>, Box<Ast>),
    Subtraction(Box<Ast>, Box<Ast>),
    Multiplication(Box<Ast>, Box<Ast>),
    Division(Box<Ast>, Box<Ast>),

    Exponent(Box<Ast>, Box<Ast>),
    Root(Box<Ast>, Box<Ast>),
}
