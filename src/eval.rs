use super::eval::types::Ast;
use lexer::Lexer;
use parser::Parser;

mod lexer;
mod parser;
mod types;

pub struct Eval;

impl Eval {
    pub fn new() -> Self {
        Self {}
    }

    pub fn eval<'a>(&self, input: &'a str) {
        let mut lexer = Lexer::new(input);
        let tokens = match lexer.lex() {
            Ok(tokens) => tokens,
            Err(e) => {
                eprintln!("{e}");
                return;
            }
        };
        let mut parser = Parser::new(&tokens);
        match parser.parse() {
            Ok(ast) => println!("{}", self.expression(&ast)),
            Err(e) => eprintln!("{e}"),
        };
    }

    fn expression(&self, node: &Ast) -> f64 {
        match node {
            Ast::Number(value) => value.clone(),
            Ast::Addition(left, right) => self.arithmetic(left, right, |x, y| x + y),
            Ast::Subtraction(left, right) => self.arithmetic(left, right, |x, y| x - y),
            Ast::Multiplication(left, right) => self.arithmetic(left, right, |x, y| x * y),
            Ast::Division(left, right) => self.arithmetic(left, right, |x, y| x / y),
            Ast::Exponent(left, right) => self.arithmetic(left, right, |x, y| x.powf(y)),
            Ast::Root(left, right) => self.arithmetic(left, right, |x, y| x.powf(1.0 / y)),
        }
    }

    fn arithmetic<F>(&self, left: &Ast, right: &Ast, operation: F) -> f64
    where
        F: Fn(f64, f64) -> f64,
    {
        let x = match left {
            Ast::Number(value) => value.clone(),
            _ => self.expression(&left),
        };
        let y = match right {
            Ast::Number(value) => value.clone(),
            _ => self.expression(&right),
        };

        operation(x, y)
    }
}
