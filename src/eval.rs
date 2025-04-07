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
        let tokens = lexer.lex();
        let mut parser = Parser::new(&tokens);
        match parser.parse() {
            Ok(ast) => println!("{}", self.expression(&ast)),
            Err(e) => eprintln!("{e}"),
        };
    }

    fn expression(&self, node: &Ast) -> f64 {
        match node {
            Ast::Number(value) => value.clone(),
            Ast::Addition(left, right) => self.addition(left, right),
            Ast::Subtraction(left, right) => self.subtraction(left, right),
            Ast::Multiplication(left, right) => self.multiplication(left, right),
            Ast::Division(left, right) => self.division(left, right),
        }
    }

    fn addition(&self, left: &Ast, right: &Ast) -> f64 {
        let x = match left {
            Ast::Number(value) => value.clone(),
            _ => self.expression(&left),
        };
        let y = match right {
            Ast::Number(value) => value.clone(),
            _ => self.expression(&right),
        };

        x + y
    }

    fn subtraction(&self, left: &Ast, right: &Ast) -> f64 {
        let x = match left {
            Ast::Number(value) => value.clone(),
            _ => self.expression(&left),
        };
        let y = match right {
            Ast::Number(value) => value.clone(),
            _ => self.expression(&right),
        };

        x - y
    }

    fn multiplication(&self, left: &Ast, right: &Ast) -> f64 {
        let x = match left {
            Ast::Number(value) => value.clone(),
            _ => self.expression(&left),
        };
        let y = match right {
            Ast::Number(value) => value.clone(),
            _ => self.expression(&right),
        };

        x * y
    }

    fn division(&self, left: &Ast, right: &Ast) -> f64 {
        let x = match left {
            Ast::Number(value) => value.clone(),
            _ => self.expression(&left),
        };
        let y = match right {
            Ast::Number(value) => value.clone(),
            _ => self.expression(&right),
        };

        x / y
    }
}
