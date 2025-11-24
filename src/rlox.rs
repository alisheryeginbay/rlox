use std::io::{Write, stdin, stdout};
use thiserror::Error;

use crate::{
    ast_printer::AstPrinter,
    interpreter::{Interpreter, RuntimeError},
    parser::{ParseError, Parser},
    scanner::{ScanError, Scanner},
};

pub struct Rlox {
    interpreter: Interpreter,
}

#[derive(Error, Debug)]
pub enum RloxError {
    #[error("scan errors: {0}")]
    Scan(#[from] ScanError),

    #[error("parse error: {0}")]
    Parse(#[from] ParseError),

    #[error("runtime error: {0}")]
    Runtime(#[from] RuntimeError),
}

impl Rlox {
    pub fn new() -> Self {
        Self {
            interpreter: Interpreter::new(),
        }
    }

    pub fn run(&mut self, source: &str) -> Result<(), Vec<RloxError>> {
        let mut scanner = Scanner::new(source.to_string());

        let tokens = scanner
            .scan_tokens()
            .map_err(|errors| errors.into_iter().map(RloxError::Scan).collect::<Vec<_>>())?;

        let mut parser = Parser::new(tokens);
        let expr = parser
            .parse()
            .map_err(|errors| errors.into_iter().map(RloxError::Parse).collect::<Vec<_>>())?;

        self.interpreter
            .interpret(expr)
            .map_err(|error| vec![RloxError::Runtime(error)])?;

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

    pub fn run(&mut self) {
        loop {
            print!("> ");
            stdout().flush().unwrap();

            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            let input = input.trim();

            if input.is_empty() || input == "/q" {
                break;
            }

            if let Err(errors) = self.rlox.run(input) {
                for error in errors {
                    eprintln!("{}", error);
                }
            }
        }
    }
}
