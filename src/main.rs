use std::{env::args, fs, io};

fn get_source_code(filename: &str) -> Result<String, io::Error> {
    let contents = fs::read_to_string(filename)?;
    Ok(contents)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = args();
    if args.len() != 2 {
        println!("Usage: rlox [name.rlox]");
        return Ok(());
    }

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

    Ok(())
}
