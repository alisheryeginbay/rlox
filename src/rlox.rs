use std::io::{Write, stdin, stdout};

use crate::scanner::Scanner;

pub struct Rlox;

impl Rlox {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self, source: &str) -> Result<(), Box<dyn std::error::Error>> {
        let scanner = Scanner::new(source.to_string());
        let tokens = scanner.scan_tokens();
        for token in tokens {
            println!("{}", token);
        }
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
