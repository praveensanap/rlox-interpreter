use std::env::args;
use std::fs::File;
use std::io;
use std::io::Write;

#[derive(Debug)]
struct TreeType {
    base_class_name: String,
    class_name: String,
    fields: Vec<String>,
}

pub fn generate_ast(output_dir: String) -> io::Result<()> {
    define_ast(
        &output_dir,
        &"Expr".to_string(),
        &vec![
            "Binary : Box<Expr> left, Token operator, Box<Expr> right".to_string(),
            "Grouping: Box<Expr> expression".to_string(),
            "Literal : Object value".to_string(),
            "Unary : Token operator, Box<Expr> right".to_string(),
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
        let class_name = format!("{}{}", base_class_name.trim(), base_name.trim()); // BinaryExpr
        let arg_split = args.split(",");
        let mut fields = Vec::new();
        for arg in arg_split {
            let (t2type, name) = arg.trim().split_once(" ").unwrap();
            fields.push(format!("{}: {}", name, t2type.trim().to_string()));
        }

        tree_types.push(TreeType {
            base_class_name: base_class_name.trim().to_string(),
            class_name,
            fields,
        });
    }
    println!("{:?}", tree_types);

    write!(file, "\npub enum {base_name} {{\n")?;
    for t in &tree_types {
        write!(file, "    {}({}),\n", t.base_class_name, t.class_name)?;
    }
    write!(file, "}}\n")?;

    for t in &tree_types {
        write!(file, "pub struct {} {{\n", t.class_name)?;
        for f in &t.fields {
            write!(file, "    {},\n", f)?;
        }
        write!(file, "}}\n\n")?;
    }

    // trait
    write!(file, "pub trait {}Visitor<T> {{\n", base_name)?;

    for t in &tree_types {
        write!(
            file,
            "     fn visit_{}_{}(&self, expr: &{}) -> Result<T, LoxError>;\n",
            t.base_class_name.to_lowercase(),
            base_name.to_lowercase(),
            t.class_name,
        )?;
    }
    write!(file, "}}\n\n")?;

    for t in &tree_types {
        write!(file, "impl {} {{\n", t.class_name)?;
        write!(
            file,
            "fn accept<T>(&self, visitor: &dyn {}Visitor<T>) -> Result<T, LoxError> {{\n",
            base_name
        )?;
        write!(
            file,
            "    visitor.visist_{}_{}(self)\n",
            t.base_class_name.to_lowercase(),
            base_name.to_lowercase()
        )?;
        write!(file, "   }}\n")?;
        write!(file, "}}\n\n")?;
    }

    Ok(())
}

//fn define_type(file: File, base_name: String, class_name: String, fields: Vec<String>){

//}
