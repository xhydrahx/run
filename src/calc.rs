use std::io::{Write, stdin, stdout};
use crate::eval;

pub fn run() {
    let mut input = String::new();

    loop {
        print!("> ");

        input.clear();
        let _ = stdout().flush();
        stdin()
            .read_line(&mut input)
            .expect("Did not enter a string");

        eval::eval(input.trim());
    }
}
