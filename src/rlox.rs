use std::io::{Write, stdin, stdout};

use crate::{ast_printer::AstPrinter, parser::Parser, scanner::Scanner};

pub struct Rlox;

impl Rlox {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self, source: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut scanner = Scanner::new(source.to_string());

        let tokens = scanner.scan_tokens().map_err(|errors| {
            println!("Failed to scan source code");
            for error in errors {
                println!("[{}] {}", error.line, error.message);
            }
            "Scanning failed"
        })?;

        let mut parser = Parser::new(tokens);
        match parser.parse() {
            Some(expr) => {
                let ast_printer = AstPrinter;
                println!("{}", ast_printer.print(&expr));

                Ok(())
            }
            None => {
                for error in parser.errors {
                    println!("{}", error);
                }

                return Err("Parsing failed".into());
            }
        }
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
