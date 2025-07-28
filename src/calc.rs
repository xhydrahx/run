use std::io::{Write, stdin, stdout};

struct Calc {
    prompt: String,
}

impl Calc {
    fn new(prompt: String) -> Self {
        Self { prompt }
    }

    fn ask(&self) -> String {
        let mut s = String::new();
        print!("{}", self.prompt);
        let _ = stdout().flush();
        let _ = stdin().read_line(&mut s);
        s.trim().to_string()
    }
}

pub fn run() {
    let calc = Calc::new("> ".into());

    loop {
        println!("{}", calc.ask());
    }
}
