#[derive(Debug)]
pub enum Token {
    Num(f64),
    Plus,
    Minus,
    Star,
    Slash,
    Carrot,
}

impl Token {
    pub fn precedence(&self) -> u8 {
        match self {
            Token::Num(_) => 0,
            Token::Plus | Token::Minus => 1,
            Token::Star | Token::Slash => 2,
            Token::Carrot => 3,
        }
    }
}

#[derive(Debug)]
pub enum Expr {
    Num(f64),
    BinaryOp(Box<Expr>, Operator, Box<Expr>),
}

#[derive(Debug)]
pub enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Exponent,
}
