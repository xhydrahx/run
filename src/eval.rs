use super::eval::types::{Ast, TrigType};
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
                eprintln!("Error: {e}");
                return;
            }
        };
        let mut parser = Parser::new(&tokens);
        match parser.parse() {
            Ok(ast) => println!("{}", self.expression(&ast)),
            Err(e) => eprintln!("Error: {e}"),
        };
    }

    fn expression(&self, node: &Ast) -> f64 {
        match node {
            Ast::Number(value) => value.clone(),
            Ast::Addition(left, right) => self.double(left, right, |x, y| x + y),
            Ast::Subtraction(left, right) => self.double(left, right, |x, y| x - y),
            Ast::Multiplication(left, right) => self.double(left, right, |x, y| x * y),
            Ast::Division(left, right) => self.double(left, right, |x, y| x / y),
            Ast::Exponent(left, right) => self.double(left, right, |x, y| x.powf(y)),
            Ast::Root(left, right) => self.double(left, right, |x, y| x.powf(1.0 / y)),
            Ast::Sqrt(node) => self.single(node, |x| x.sqrt()),
            Ast::Log(left, right) => self.double(left, right, |x, y| y.log(x)),
            Ast::Ln(node) => self.single(node, |x| x.ln()),
            Ast::Factorial(node) => {
                let fact = |x: u64| (1..=x).fold(1i128, |acc, v| acc * v as i128);
                self.single(node, |x| {
                    if x < 0.0 {
                        (-fact((-x) as u64)) as f64
                    } else {
                        fact(x as u64) as f64
                    }
                })
            }
            Ast::Trig(trig_type, node) => self.trig(trig_type.clone(), node),
        }
    }

    fn trig(&self, trig_type: TrigType, node: &Ast) -> f64 {
        match trig_type {
            TrigType::Sin => self.single(node, |x| x.sin()),
            TrigType::Cos => self.single(node, |x| x.cos()),
            TrigType::Tan => self.single(node, |x| x.tan()),
            TrigType::Csc => self.single(node, |x| 1.0 / x.sin()),
            TrigType::Sec => self.single(node, |x| 1.0 / x.cos()),
            TrigType::Cot => self.single(node, |x| 1.0 / x.tan()),
            TrigType::Arcsin => self.single(node, |x| x.asin()),
            TrigType::Arccos => self.single(node, |x| x.acos()),
            TrigType::Arctan => self.single(node, |x| x.atan()),
            TrigType::Arccsc => self.single(node, |x| (1.0 / x.sin()).asin()),
            TrigType::Arcsec => self.single(node, |x| (1.0 / x.cos()).acos()),
            TrigType::Arccot => self.single(node, |x| (1.0 / x.tan()).atan()),
            TrigType::Sinh => self.single(node, |x| x.sinh()),
            TrigType::Cosh => self.single(node, |x| x.cosh()),
            TrigType::Tanh => self.single(node, |x| x.tanh()),
            TrigType::Coth => self.single(node, |x| 1.0 / x.tanh()),
            TrigType::Sech => self.single(node, |x| 1.0 / x.cosh()),
            TrigType::Csch => self.single(node, |x| 1.0 / x.sinh()),
            TrigType::Arcsinh => self.single(node, |x| x.asinh()),
            TrigType::Arccosh => self.single(node, |x| x.acosh()),
            TrigType::Arctanh => self.single(node, |x| x.atanh()),
            TrigType::Arccoth => self.single(node, |x| 0.5 * ((x + 1.0) / (x - 1.0)).ln()),
            TrigType::Arcsech => self.single(node, |x| (1.0 / x).acosh()),
            TrigType::Arccsch => self.single(node, |x| (1.0 / x).asinh()),
        }
    }

    fn double<F>(&self, left: &Ast, right: &Ast, operation: F) -> f64
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

    fn single<F>(&self, node: &Ast, operation: F) -> f64
    where
        F: Fn(f64) -> f64,
    {
        let x = match node {
            Ast::Number(value) => value.clone(),
            _ => self.expression(&node),
        };

        operation(x)
    }
}
