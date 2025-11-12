use anyhow::{Context, Result};
use std::path::Path;
use crate::trait_generator::GeneratedTrait;
use quote::ToTokens;
use std::fs;
use std::io::Write;

pub fn write_trait_to_file(
    output_dir: &Path,
    generated_trait: &GeneratedTrait,
    dry_run: bool, // Add dry_run argument
) -> Result<()> {
    let file_name = format!("{}.rs", generated_trait.name.to_lowercase());
    let file_path = output_dir.join(file_name);

    let tokens = generated_trait.to_token_stream();
    let syntax_tree = syn::parse_file(&tokens.to_string())
        .context(format!("Failed to parse generated trait tokens for {}", generated_trait.name))?;
    let formatted_code = prettyplease::unparse(&syntax_tree);

    if dry_run {
        println!("DRY RUN: Would write trait to {:?}", file_path);
    } else {
        let mut file = fs::File::create(&file_path)
            .context(format!("Failed to create file for trait: {:?}", file_path))?;
        file.write_all(formatted_code.as_bytes())
            .context(format!("Failed to write to file for trait: {:?}", file_path))?;

        println!("Successfully wrote trait to {:?}", file_path);
    }
    Ok(())
}
