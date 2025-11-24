use std::{
    env::args,
    fs,
    io::{self},
};

use crate::rlox::Repl;

mod ast_printer;
mod expr;
mod interpreter;
mod parser;
mod rlox;
mod scanner;
mod token;

fn get_source_code(filename: &str) -> Result<String, io::Error> {
    let contents = fs::read_to_string(filename)?;
    Ok(contents)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = args();

    if args.len() > 2 {
        println!("Usage: rlox [name.rlox]");
        return Ok(());
    }

    let rlox = rlox::Rlox::new();

    if args.len() == 1 {
        let repl = Repl::new(rlox);
        repl.run();
        return Ok(());
    }

    let source_code_filename = args.into_iter().nth(1).unwrap();
    if !source_code_filename.ends_with(".rlox") {
        eprintln!("Error: Invalid file extension. Expected .rlox");
        return Ok(());
    }

    println!("Source code filename: {}", source_code_filename);

    let contents = match get_source_code(source_code_filename.as_str()) {
        Ok(v) => v,
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                eprintln!("File {} not found", source_code_filename.clone());
                return Ok(());
            } else {
                eprintln!("Error reading file: {}", e);
                return Err(e.into());
            }
        }
    };

    if let Err(errors) = rlox.run(contents.as_str()) {
        for error in errors {
            eprintln!("{}", error);
        }
    }

    Ok(())
}
