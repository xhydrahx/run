use crate::eval::Eval;
use std::io::{Write, stdin, stdout};

struct Calc {
    prompt: String,
}

impl Calc {
    fn new(prompt: &str) -> Self {
        Calc {
            prompt: prompt.to_string(),
        }
    }

    fn input(&self) -> String {
        print!("{} ", self.prompt);

        let mut s = String::new();
        let _ = stdout().flush();
        stdin()
            .read_line(&mut s)
            .expect("Did not type in correct string");
        s.trim().to_string()
    }
}

pub fn run() {
    let calc = Calc::new(">");
    let eval = Eval::new();
    loop {
        eval.eval(&calc.input());
    }
}
