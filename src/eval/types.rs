#[derive(Debug)]
pub enum Token {
    Number(f64),

    Plus,
    Minus,
    Multiply,
    Divide,
}
