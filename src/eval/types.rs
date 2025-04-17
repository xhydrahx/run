#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(f64),

    Addition,
    Subtraction,
    Multiplication,
    Division,

    Exponent,
    Root,
    Sqrt,

    Log,
    Ln,

    Sin,
    Cos,
    Tan,

    Factorial,

    LeftParen,
    RightParen,

    Comma,
    Underscore,

    E,
    Pi,
    Phi,
}

impl Token {
    pub fn is_unary(&self) -> bool {
        match self {
            Token::Factorial => true,
            _ => false,
        }
    }
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
    Sqrt(Box<Ast>),

    Log(Box<Ast>, Box<Ast>),
    Ln(Box<Ast>),

    Sin(Box<Ast>),
    Cos(Box<Ast>),
    Tan(Box<Ast>),

    Factorial(Box<Ast>),
}
