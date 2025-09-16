use std::{
    env::args,
    fs,
    io::{self, Write},
};

fn get_source_code(filename: &str) -> Result<String, io::Error> {
    let contents = fs::read_to_string(filename)?;
    Ok(contents)
}

struct Repl {}

impl Repl {
    fn new() -> Self {
        Repl {}
    }

    fn run(&self) {
        loop {
            print!("> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            let input = input.trim();

            if input.is_empty() || input == "/q" {
                break;
            }

            println!("Source code:\n{}", input);
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = args();

    if args.len() > 2 {
        println!("Usage: rlox [name.rlox]");
        return Ok(());
    }

    if args.len() == 2 {
        let source_code_filename = args.into_iter().nth(1).unwrap();
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

        println!("Source code:\n{}", contents);
        return Ok(());
    }

    let repl = Repl::new();
    repl.run();

    Ok(())
}
