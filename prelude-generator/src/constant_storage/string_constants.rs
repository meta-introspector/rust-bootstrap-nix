use anyhow::{Context, Result};
use std::path::PathBuf;
use quote::quote;
use tokio::fs;

const MAX_FILE_SIZE: usize = 4 * 1024; // 4KB

pub async fn write_string_constants_to_hierarchical_structure(
    constants: &[syn::ItemConst],
    output_dir: &PathBuf,
) -> anyhow::Result<()> {
    println!("  -> Writing string constants to hierarchical structure...");
    let mut current_file_size = 0;
    let mut file_idx = 0;
    let mut current_file_content = String::new();

    for constant in constants {
        let const_name = constant.ident.to_string();
        let const_code = quote! { #constant }.to_string();
        let line = format!("{}\n// 8D_EMBEDDING: 0\n", const_code);

        if current_file_size + line.len() > MAX_FILE_SIZE && current_file_size > 0 {
            // Write current_file_content to a file
            let file_path = output_dir.join(format!("string_constants_{}.rs", file_idx));
            tokio::fs::create_dir_all(file_path.parent().unwrap()).await?;
            tokio::fs::write(&file_path, current_file_content.as_bytes()).await?;
            println!("    -> Wrote string constants to {:?}\n", file_path);

            // Reset for next file
            current_file_content.clear();
            current_file_size = 0;
            file_idx += 1;
        }

        current_file_content.push_str(&line);
        current_file_size += line.len();
    }

    // Write any remaining content
    if current_file_size > 0 {
        let file_path = output_dir.join(format!("string_constants_{}.rs", file_idx));
        tokio::fs::create_dir_all(file_path.parent().unwrap()).await?;
        tokio::fs::write(&file_path, current_file_content.as_bytes()).await?;
        println!("    -> Wrote string constants to {:?}\n", file_path);
    }

    Ok(())
}
