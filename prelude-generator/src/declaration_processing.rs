use anyhow::Context;
use std::path::{PathBuf, Path};
use syn::visit::Visit;
use quote::quote;

use crate::decls_visitor::DeclsVisitor;
use std::collections::HashMap;
use crate::use_extractor::expand_macros_and_parse;
use crate::use_extractor::rustc_info::RustcInfo;
use crate::use_statements;
use crate::utils;
use crate::type_extractor;

pub async fn extract_level0_declarations(
    project_root: &PathBuf,
    _args: &crate::Args,
    type_map: &HashMap<String, type_extractor::TypeInfo>,
    filter_names: &Option<Vec<String>>,
    rustc_info: &RustcInfo,
    cache_dir: &Path,
) -> anyhow::Result<(
    Vec<syn::ItemConst>,
    HashMap<usize, Vec<syn::ItemStruct>>,
    usize, // total_files_processed
    usize,
    usize,
    usize,
    usize,
    usize,
    HashMap<usize, usize>,
)> {
    let src_dir = project_root.join("src");
    println!("Attempting to read directory: {}", src_dir.display());
    let mut all_constants: Vec<syn::ItemConst> = Vec::new();
    let mut all_structs_by_layer: HashMap<usize, Vec<syn::ItemStruct>> = HashMap::new();
    let mut total_files_processed = 0;
    let mut total_fns = 0;
    let mut total_structs = 0;
    let mut total_enums = 0;
    let mut total_statics = 0;
    let mut total_other_items = 0;
    let mut total_structs_per_layer: HashMap<usize, usize> = HashMap::new();

    for entry in std::fs::read_dir(&src_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
        .filter(|e| {
            if let Some(names) = filter_names {
                names.iter().any(|name| e.file_name().to_string_lossy().contains(name))
            } else {
                true
            }
        })
    {
        let path = entry.path();

        if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
            total_files_processed += 1;
            println!("  Processing file: {}", path.display());
            let content = std::fs::read_to_string(&path)?;
            let mut writer = tokio::io::stdout(); // Or a specific log file writer
            let file = expand_macros_and_parse(&mut writer, &path, &content, rustc_info, cache_dir).await
                .with_context(|| format!("Failed to expand macros and parse file: {}", path.display()))?;

            let mut visitor = DeclsVisitor::new();
            visitor.visit_file(&file);

            // Filter structs based on layer information
            for structure in visitor.all_structs {
                let struct_name = structure.ident.to_string();
                let layer = type_map.get(&struct_name).and_then(|info| info.layer).unwrap_or(0);
                all_structs_by_layer.entry(layer).or_insert_with(Vec::new).push(structure);
                *total_structs_per_layer.entry(layer).or_insert(0) += 1;
            }

            all_constants.extend(visitor.constants);

            total_fns += visitor.fn_count;
            total_structs += visitor.struct_count;
            total_enums += visitor.enum_count;
            total_statics += visitor.static_count;
            total_other_items += visitor.other_item_count;
        }
    }

    Ok((
        all_constants,
        all_structs_by_layer,
        total_files_processed,
        total_fns,
        total_structs,
        total_enums,
        total_statics,
        total_other_items,
        total_structs_per_layer,
    ))
}

pub async fn process_constants(
    all_constants: Vec<syn::ItemConst>,
    _args: &crate::Args,
    project_root: &PathBuf,
    _all_numerical_constants: &mut Vec<syn::ItemConst>,
    _all_string_constants: &mut Vec<syn::ItemConst>,
    type_map: &HashMap<String, type_extractor::TypeInfo>,
) -> anyhow::Result<()> {
    let generated_decls_output_dir = _args.generated_decls_output_dir.clone().unwrap_or_else(|| {
        project_root.join("generated/level0_decls")
    });

    let numerical_output_dir = project_root.join("generated/numerical_constants");
    println!("Attempting to create numerical constants output directory: {}", numerical_output_dir.display());
    tokio::fs::create_dir_all(&numerical_output_dir).await?;


    let string_output_dir = project_root.join("generated/string_constants");
    println!("Attempting to create string constants output directory: {}", string_output_dir.display());
    tokio::fs::create_dir_all(&string_output_dir).await?;


    println!("  -> Generated constants will be written to layer-specific directories.");

    let mut errors: Vec<anyhow::Error> = Vec::new();

    for constant in &all_constants {
        let const_name = constant.ident.to_string();
        let layer = type_map.get(&const_name).and_then(|info| info.layer).unwrap_or(0);
        let consts_output_dir = generated_decls_output_dir.join(format!("layer_{}", layer)).join("const");
        println!("Attempting to create directory: {}", consts_output_dir.display());
        tokio::fs::create_dir_all(&consts_output_dir).await
            .context(format!("Failed to create output directory {:?}", consts_output_dir))?;

        let file_name = format!("{}.rs", const_name);
        let output_path = consts_output_dir.join(&file_name);
        println!("Attempting to write file: {}", output_path.display());
        let result = async {
            let tokens = quote! { #constant };
            let mut code = tokens.to_string();

            let required_uses = use_statements::get_required_uses_for_item_const(&constant);
            code = format!("{}{}", required_uses, code);

            tokio::fs::write(&output_path, code.as_bytes()).await
                .context(format!("Failed to write constant {:?} to {:?}", const_name, output_path))?;
            println!("  -> Wrote constant {:?} to {:?}", const_name, output_path);

            // Format the generated code
            utils::format_rust_code(&output_path).await
                .context(format!("Constant {:?} formatting failed for {:?}", const_name, output_path))?;
            println!("  -> Constant {:?} formatted successfully.\n", const_name);

            // Validate the generated code
            utils::validate_rust_code(&output_path).await
                .context(format!("Constant {:?} validation failed for {:?}", const_name, output_path))?;
            println!(r"  -> Constant {:?} validated successfully.\n", const_name);
            Ok(())
        }.await;

        if let Err(e) = result {
            eprintln!(r"Error processing constant {}: {:?}\n", const_name, e);
            errors.push(e);
        }
    }

    if !errors.is_empty() {
        eprintln!(r"\n--- Errors Encountered during constant processing ---");
        for error in &errors {
            eprintln!(r"{:?}", error);
        }
        eprintln!(r"-----------------------------------------------------");
        return Err(anyhow::anyhow!("Constant processing completed with errors."));
    } else {
        println!(r"Declaration processing completed successfully.");
        return Ok(())
    }
}

pub async fn process_structs(
    all_structs_by_layer: HashMap<usize, Vec<syn::ItemStruct>>,
    args: &crate::Args,
    project_root: &PathBuf,
    _type_map: &HashMap<String, type_extractor::TypeInfo>,
) -> anyhow::Result<()> {
    let generated_decls_output_dir = args.generated_decls_output_dir.clone().unwrap_or_else(|| {
        project_root.join("generated/level0_decls")
    });

    println!("  -> Generated structs will be written to layer-specific directories.");

    let mut errors: Vec<anyhow::Error> = Vec::new();

    // Only process structs for layer 0 for now, as per requirement
    if let Some(layer0_structs) = all_structs_by_layer.get(&0) {
        for structure in layer0_structs {
            let struct_name = structure.ident.to_string();
            let layer = 0; // Explicitly set to 0 for Level 0 processing
            let structs_output_dir = generated_decls_output_dir.join(format!("layer_{}", layer)).join("struct");
            println!("Attempting to create directory: {}", structs_output_dir.display());
            tokio::fs::create_dir_all(&structs_output_dir).await
                .context(format!("Failed to create output directory {:?}, for struct {}", structs_output_dir, struct_name))?;

            let file_name = format!("{}.rs", struct_name);
            let output_path = structs_output_dir.join(&file_name);
            println!("Attempting to write file: {}", output_path.display());
            let content = quote::quote! { #structure }.to_string();
            let code = match struct_name.as_str() {
                "DeclsVisitor" => {
                    format!("use syn::{{visit::Visit, ItemConst, ItemFn, ItemStruct, ItemEnum, ItemStatic, Item}};
{}", content)
                },
                "TypeCollector" => {
                    format!("use std::collections::HashMap;
use crate::type_extractor::TypeInfo;
{}", content)
                },
                _ => content,
            };

            let result = async {
                tokio::fs::write(&output_path, code.as_bytes()).await
                    .context(format!("Failed to write struct {:?} to {:?}", struct_name, output_path))?;
                println!("  -> Wrote struct {:?} to {:?}", struct_name, output_path);

                // Format the generated code
                utils::format_rust_code(&output_path).await
                    .context(format!("Struct {:?} formatting failed for {:?}", struct_name, output_path))?;
                println!("  -> Struct {:?} formatted successfully.\n", struct_name);

                // Validate the generated code
                utils::validate_rust_code(&output_path).await
                    .context(format!("Struct {:?} validation failed for {:?}", struct_name, output_path))?;
                println!(r"  -> Struct {:?} validated successfully.\n", struct_name);
                Ok(())
            }.await;

            if let Err(e) = result {
                eprintln!(r"Error processing struct {}: {:?}\n", struct_name, e);
                errors.push(e);
            }
        }
    } else {
        println!("No Level 0 structs found to process.");
    }

    if !errors.is_empty() {
        eprintln!(r"\n--- Errors Encountered during struct processing ---");
        for error in &errors {
            eprintln!(r"{:?}", error);
        }
        eprintln!(r"---------------------------------------------------");
        return Err(anyhow::anyhow!("Struct processing completed with errors."));
    } else {
        println!(r"Declaration processing completed successfully.");
        return Ok(())
    }
}

pub fn generate_constants_module(constants: &[syn::ItemConst]) -> String {
    let generated_decl_strings: Vec<String> = constants.iter().map(|c| {
        let tokens = quote! { #c };
        tokens.to_string()
    }).collect();

    if generated_decl_strings.is_empty() {
        return "// No constant declarations found in this module.\n".to_string();
    }

    let header = "// This module contains extracted constant declarations.\n// It is automatically generated.\n\n";
    let joined_decls = generated_decl_strings.join("\n\n");

    format!("{}{}", header, joined_decls)
}

pub fn generate_structs_module(structs: &[syn::ItemStruct]) -> String {
    let generated_decl_strings: Vec<String> = structs.iter().map(|s| {
        let tokens = quote! { #s };
        tokens.to_string()
    }).collect();

    if generated_decl_strings.is_empty() {
        return "// No struct declarations found in this module.\n".to_string();
    }

    let header = "// This module contains extracted struct declarations.\n// It is automatically generated.\n\n";
    let joined_decls = generated_decl_strings.join("\n\n");

    format!("{}{}", header, joined_decls)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Args;
    use crate::type_extractor::{self};
    use std::collections::HashMap;
    use std::path::PathBuf;
    use tokio;

    // Helper function to set up a minimal test project
    async fn setup_test_project(test_dir_name: &str) -> anyhow::Result<PathBuf> {
        let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("temp_test_projects").join(test_dir_name);
        let src_dir = project_root.join("src");
        tokio::fs::create_dir_all(&src_dir).await?;

        let cargo_toml_content = r#"#
[package]
name = "test_project"
version = "0.1.0"
edition = "2021"

[lib]
path = "src/lib.rs"
#"#;
        tokio::fs::write(project_root.join("Cargo.toml"), cargo_toml_content).await?;

        let lib_rs_content = r#"#
pub const TEST_NUM_CONST: u32 = 123;
pub const TEST_STR_CONST: &str = "hello";

pub struct TestStruct {
    pub field1: u32,
    field2: String,
}

pub enum TestEnum {
    Variant1,
    Variant2(u32),
}

pub fn test_function() -> u32 {
    TEST_NUM_CONST
}
#"#;
        tokio::fs::write(src_dir.join("lib.rs"), lib_rs_content).await?;

        Ok(project_root)
    }

    #[tokio::test]
    async fn test_extract_level0_declarations_minimal() -> anyhow::Result<()> {
        let test_project_root = setup_test_project("minimal_project").await?;

        let args = Args {
            path: test_project_root.clone(),
            ..Default::default()
        };

        // Extract type map first
        let type_map = type_extractor::extract_bag_of_types(&test_project_root, &None).await?;

        // Call the function under test
        let (constants, structs, total_files_processed, fns, s_structs, enums, statics, other_items, l0_structs) =
            extract_level0_declarations(&test_project_root, &args, &type_map, &None).await?;

        // Assertions
        assert_eq!(total_files_processed, 1);
        assert_eq!(constants.len(), 2); // TEST_NUM_CONST, TEST_STR_CONST
        assert_eq!(structs.len(), 1); // TestStruct
        assert_eq!(enums, 1); // TestEnum
        assert_eq!(fns, 1); // test_function
        assert_eq!(s_structs, 1); // Assert that s_structs is 1
        assert_eq!(statics, 0); // Assert that statics is 0
        assert_eq!(other_items, 0); // Assert that other_items is 0
        assert_eq!(l0_structs, 1); // Assert that l0_structs is 1

        // Clean up the temporary project
        tokio::fs::remove_dir_all(&test_project_root).await?;

        Ok(())
    }
}