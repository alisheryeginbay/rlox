use std::io::{Write, stdin, stdout};

pub struct Rlox;

impl Rlox {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self, source: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("Running source code:");
        println!("{}", source);
        Ok(())
    }
}

pub struct Repl {
    rlox: Rlox,
}

impl Repl {
    pub fn new(rlox: Rlox) -> Self {
        Repl { rlox }
    }

    pub fn run(&self) {
        loop {
            print!("> ");
            stdout().flush().unwrap();

            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            let input = input.trim();

            if input.is_empty() || input == "/q" {
                break;
            }

            match self.rlox.run(input) {
                Ok(_) => println!("Source code executed successfully"),
                Err(e) => eprintln!("Error executing source code: {}", e),
            }
        }
    }
}
