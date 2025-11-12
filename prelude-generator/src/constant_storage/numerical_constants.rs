use anyhow::Result;
use std::path::PathBuf;
use quote::quote;
use syn::ItemConst;

const MAX_FILE_SIZE: usize = 4 * 1024; // 4KB

pub async fn write_numerical_constants_to_hierarchical_structure(
    constants: &[ItemConst],
    output_dir: &PathBuf,
) -> Result<()> {
    println!("  -> Writing numerical constants to hierarchical structure...");
    let mut current_file_size = 0;
    let mut file_idx = 0;
    let mut current_file_content = String::new();

    for constant in constants {
        let _const_name = constant.ident.to_string();
        let const_code = quote! { #constant }.to_string();
        let line = format!("{}\n", const_code);

        if current_file_size + line.len() > MAX_FILE_SIZE && current_file_size > 0 {
            // Write current_file_content to a file
            let file_path = output_dir.join(format!("numerical_constants_{}.rs", file_idx));
            tokio::fs::create_dir_all(file_path.parent().unwrap()).await?;
            tokio::fs::write(&file_path, current_file_content.as_bytes()).await?;
            println!("    -> Wrote numerical constants to {:?}\n", file_path);

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
        let file_path = output_dir.join(format!("numerical_constants_{}.rs", file_idx));
        tokio::fs::create_dir_all(file_path.parent().unwrap()).await?;
        tokio::fs::write(&file_path, current_file_content.as_bytes()).await?;
        println!("    -> Wrote numerical constants to {:?}\n", file_path);
    }

    Ok(())
}