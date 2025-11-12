use anyhow::Result;
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::fs;
use walkdir::WalkDir;
use syn::{Lit, Type};

pub async fn generate_numerical_constants_report(
    output_dir: &PathBuf,
) -> Result<()> {
    println!("  -> Generating numerical constants report from directory: {}", output_dir.display());

    let mut constants_by_type_and_size: HashMap<String, HashMap<String, usize>> = HashMap::new();

    for entry in WalkDir::new(output_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
            println!("    -> Processing file: {}", path.display());
            let content = fs::read_to_string(path).await?;
            // println!("       File content (first 200 chars): {}", &content[..std::cmp::min(content.len(), 200)]);

            match syn::parse_file(&content) {
                Ok(file) => {
                    println!("       Successfully parsed file: {}", path.display());
                    for item in file.items {
                        if let syn::Item::Const(constant) = item {
                            let type_name = get_type_name(&constant.ty);
                            let value_str = get_constant_value_string(&constant.expr);
                            println!("         Found constant: {} (type: {}, value: {})", constant.ident, type_name, value_str);

                            if !type_name.is_empty() && !value_str.is_empty() {
                                *constants_by_type_and_size
                                    .entry(type_name)
                                    .or_default()
                                    .entry(value_str)
                                    .or_insert(0) += 1;
                            }
                        }
                    }
                },
                Err(e) => {
                    eprintln!("       Failed to parse file {}: {}", path.display(), e);
                }
            }
        }
    }

    println!("\n--- Numerical Constants Report ---");
    if constants_by_type_and_size.is_empty() {
        println!("No numerical constants found.");
    } else {
        for (type_name, values) in &constants_by_type_and_size {
            println!("Type: {}", type_name);
            let mut sorted_values: Vec<(&String, &usize)> = values.iter().collect();
            sorted_values.sort_by(|a, b| a.0.cmp(b.0)); // Sort by value string

            for (value, count) in sorted_values {
                println!("  Value: {}, Count: {}", value, count);
            }
        }
    }
    println!("----------------------------------");

    Ok(())
}

fn get_type_name(ty: &Box<Type>) -> String {
    if let Type::Path(type_path) = &**ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident.to_string();
        }
    }
    String::new()
}

fn get_constant_value_string(expr: &Box<syn::Expr>) -> String {
    if let syn::Expr::Lit(expr_lit) = &**expr {
        match &expr_lit.lit {
            Lit::Int(lit_int) => lit_int.base10_digits().to_string(),
            Lit::Float(lit_float) => lit_float.base10_digits().to_string(),
            _ => String::new(),
        }
    } else {
        String::new()
    }
}