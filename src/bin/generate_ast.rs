use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

fn main() {
    let mut descriptions = HashMap::new();
    descriptions.insert("Binary", "left: Rc<Expr>, operator: Token, right: Rc<Expr>");
    descriptions.insert("Grouping", "expression: Rc<Expr>");
    descriptions.insert("Literal", "value: Literal");
    descriptions.insert("Unary", "operator: Token, right: Rc<Expr>");

    match define_ast("Expr", descriptions) {
        Ok(()) => {
            println!("Generated Expr enum");
        }
        Err(err) => {
            eprintln!("Failed to generate Expr enum: {}", err);
        }
    }
}

const IMPORTS: &[&str] = &["token"];

fn define_type(
    mut writer: impl Write,
    name: &str,
    description: &str,
) -> Result<(), std::io::Error> {
    writeln!(writer, "\t{} {{", name)?;

    let fields: Vec<&str> = description
        .split_terminator(",")
        .map(|s| s.trim())
        .collect();

    for field in fields {
        writeln!(writer, "\t\t{},", field)?;
    }

    writeln!(writer, "\t}},")?;
    Ok(())
}

fn define_ast(base_name: &str, descriptions: HashMap<&str, &str>) -> Result<(), std::io::Error> {
    let output_path = format!("src/{}.rs", base_name);
    let mut file = File::create(output_path)?;

    for import in IMPORTS {
        writeln!(file, "use crate::{}::*;", import)?;
    }

    writeln!(file, "use std::rc::Rc;")?;

    writeln!(file)?;

    writeln!(file, "pub enum {} {{", base_name)?;

    for (name, description) in descriptions {
        define_type(&file, name, description)?;
    }

    writeln!(file, "}}")?;

    Ok(())
}
