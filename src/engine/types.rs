#[derive(Debug, Clone)]
pub enum Token {
    Num(f64),
    Identifier(String),
    Plus,
    Minus,
    Star,
    Slash,
    Carrot,
    LeftParen,
    RightParen,
    Exclamation,
}

impl Token {
    pub fn precedence(&self) -> u8 {
        match self {
            Token::Plus | Token::Minus => 1,
            Token::Star | Token::Slash => 2,
            Token::Carrot => 3,
            Token::Exclamation => 4,
            _ => 0,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            Token::Num(n) => &n.to_string(),
            Token::Identifier(i) => &i.to_string(),
            Token::Plus => "+",
            Token::Minus => "-",
            Token::Star => "*",
            Token::Slash => "/",
            Token::Carrot => "^",
            Token::LeftParen => "(",
            Token::RightParen => ")",
            Token::Exclamation => "!",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub enum Expr {
    Num(f64),
    Binary(Box<Expr>, Operator, Box<Expr>),
    Unary(Operator, Box<Expr>),
}

#[derive(Debug)]
pub enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Exponent,
    Factorial(i8),
}
