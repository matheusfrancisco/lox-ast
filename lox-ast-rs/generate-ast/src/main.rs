use std::env::args;
use std::fs::File;
use std::io::{self, Write};

#[derive(Debug)]
struct TreeType {
    base_class_name: String,
    class_name: String,
    fields: Vec<String>,
}

fn main() -> io::Result<()> {
    let args: Vec<String> = args().collect();

    if args.len() != 2 {
        eprintln!("Usage: generate-ast <output directory>");
        std::process::exit(64);
    }

    let output_dir = args.get(1).unwrap().to_string();

    define_ast(
        &output_dir,
        &"Expr".to_string(),
        &vec![
            "Binary   : left: Box<Expr>, operator: Token, right: Box<Expr>".to_string(),
            "Grouping : expression: Box<Expr>".to_string(),
            "Literal  : value: Literal".to_string(),
            "Unary    : operator: Token, right: Box<Expr>".to_string(),
        ],
    )?;
    Ok(())
}

fn define_ast(output_dir: &String, base_name: &String, types: &[String]) -> io::Result<()> {
    let path = format!("{output_dir}/{}.rs", base_name.to_lowercase());
    let mut file = File::create(path)?;
    let mut tree_types = Vec::new();

    write!(file, "{}", "use crate::error::*;\n")?;
    write!(file, "{}", "use crate::token::*;\n")?;

    for ttype in types {
        let (base_class_name, args) = ttype.split_once(":").unwrap();
        let class_name = format!("{}{}", base_class_name, base_name); // Binary + Expr
        let arg_split = args.split(",");
        let mut fields = Vec::new();

        for arg in arg_split {
            let (t2type, name) = arg.trim().split_once(" ").unwrap();
            fields.push(format!("{}: {}", name, t2type));
        }
        tree_types.push(TreeType {
            base_class_name: base_class_name.trim().to_string(),
            class_name,
            fields,
        });
    }

    writeln!(file, "\npub enum {base_name} {{")?;
    for t in &tree_types {
        writeln!(file, "    {}(Rc<{}>),", t.base_class_name, t.class_name)?;
    }
    writeln!(file, "}}\n")?;

    for t in tree_types {
        writeln!(file, "pub struct {} {{", t.class_name)?;
    }

    Ok(())
}
