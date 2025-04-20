use crate::engine;
use std::io::{Write, stdin, stdout};

pub fn run() {
    let mut input = String::new();

    loop {
        print!("> ");

        input.clear();
        let _ = stdout().flush();
        stdin()
            .read_line(&mut input)
            .expect("Did not enter a string");

        match engine::expr(input.trim()) {
            Ok(n) => println!("=> {}", n),
            Err(e) => eprintln!("=> {}", e),
        }
    }
}
