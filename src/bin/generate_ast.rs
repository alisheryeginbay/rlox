use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

fn main() {
    let mut expressions = HashMap::new();
    expressions.insert(
        "Binary",
        "left: Box<Expr>, operator: Token, right: Box<Expr>",
    );
    expressions.insert("Grouping", "expression: Box<Expr>");
    expressions.insert("Literal", "value: Literal");
    expressions.insert("Unary", "operator: Token, right: Box<Expr>");
    expressions.insert(
        "Ternary",
        "condition: Box<Expr>, positive: Box<Expr>, negative: Box<Expr>",
    );

    let mut statements = HashMap::new();
    statements.insert("Expression", "expression: Box<Expr>");
    statements.insert("Print", "expression: Box<Expr>");

    match define_ast("Expr", expressions) {
        Ok(()) => {
            println!("Generated Expr enum");
        }
        Err(err) => {
            eprintln!("Failed to generate Expr enum: {}", err);
        }
    }

    match define_ast("Stmt", statements) {
        Ok(()) => {
            println!("Generated Stmt enum");
        }
        Err(err) => {
            eprintln!("Failed to generate Stmt enum: {}", err);
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

fn define_ast(base_name: &str, expressions: HashMap<&str, &str>) -> Result<(), std::io::Error> {
    let output_path = format!("src/{}.rs", base_name.to_lowercase());
    let mut file = File::create(output_path)?;

    for import in IMPORTS {
        writeln!(file, "use crate::{}::*;", import)?;
    }

    writeln!(file)?;

    writeln!(file, "pub enum {} {{", base_name)?;

    for (name, description) in expressions.iter() {
        define_type(&file, name, description)?;
    }

    writeln!(file, "}}\n")?;

    Ok(())
}
