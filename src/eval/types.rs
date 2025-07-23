#[derive(Debug, Clone, PartialEq)]
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
    Percent,
    Bar,
    Equal,

    Comma,
    Underscore,
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
            Token::Percent => "%",
            Token::Comma => ",",
            Token::Underscore => "_",
            Token::Bar => "|",
            Token::Equal => "=",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    Num(f64),
    Var(String, Box<Expr>),
    Func(String, Vec<Box<Expr>>),
    Bin(Box<Expr>, Operator, Box<Expr>),
    Unary(Operator, Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Exponent,
    Factorial(i8),
    Percent,
    Absolute,
    Equal
}
