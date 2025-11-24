use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let expressions = HashMap::from([
        (
            "Binary",
            "left: Box<Expr>, operator: Token, right: Box<Expr>",
        ),
        ("Grouping", "expression: Box<Expr>"),
        ("Literal", "value: Literal"),
        ("Unary", "operator: Token, right: Box<Expr>"),
        (
            "Ternary",
            "condition: Box<Expr>, positive: Box<Expr>, negative: Box<Expr>",
        ),
    ]);

    let statements = HashMap::from([
        ("Expression", "expression: Box<Expr>"),
        ("Print", "expression: Box<Expr>"),
    ]);

    define_ast("Expr", expressions)?;
    println!("Generated Expr enum");

    define_ast("Stmt", statements)?;
    println!("Generated Stmt enum");

    Ok(())
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
