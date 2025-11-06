use anyhow::{anyhow, Result};
use syn::UseTree;
use crate::config_parser::Config;
//use crate::external_interfaces::{SynInterface, ExternalInterfaceGateway};
//use crate::external_interfaces::ExternalInterfaceGateway;

// pub fn verify_module_exports(generated_code: &str, config: &Config, gateway: &ExternalInterfaceGateway) -> Result<()> {
//     let parsed_file = gateway.syn_interface.parse_file(generated_code)
//         .map_err(|e| anyhow!("Failed to parse generated code: {}", e))?;

//     let mut actual_exports = Vec::new();
//     for item in parsed_file.items {
//         if let syn::Item::Use(use_item) = item {
//             extract_use_paths(&use_item.tree, &mut actual_exports, String::new());
//         }
//     }

//     let mut expected_exports = Vec::new();
//     if let Some(module_exports_config) = &config.module_exports {
//         if let Some(modules) = &module_exports_config.modules {
//             expected_exports.extend(modules.iter().cloned());
//         }
//     }

//     // Check for missing exports
//     for expected in &expected_exports {
//         if !actual_exports.contains(expected) {
//             return Err(anyhow!("Missing expected export: {}", expected));
//         }
//     }

//     // Check for unexpected exports
//     for actual in &actual_exports {
//         if !expected_exports.contains(actual) {
//             return Err(anyhow!("Unexpected export found: {}", actual));
//         }
//     }

//     println!("Module exports verified successfully.");
//     Ok(())
// }

fn extract_use_paths(use_tree: &UseTree, paths: &mut Vec<String>, current_path: String) {
    match use_tree {
        UseTree::Path(path) => {
            let new_path = if current_path.is_empty() {
                path.ident.to_string()
            } else {
                format!("{}::{}", current_path, path.ident)
            };
            extract_use_paths(&path.tree, paths, new_path);
        }
        UseTree::Name(name) => {
            paths.push(format!("{}::{}", current_path, name.ident));
        }
        UseTree::Rename(rename) => {
            paths.push(format!("{}::{}", current_path, rename.ident));
        }
        UseTree::Glob(_) => {
            // Handle glob imports if necessary, for now, we'll just ignore them for exact matching
            // Or, we could expand them if we have access to the module structure
            // For simplicity, we'll treat them as a potential mismatch for exact verification
            paths.push(format!("{}::*", current_path));
        }
        UseTree::Group(group) => {
            for (i, item) in group.items.iter().enumerate() {
                extract_use_paths(item, paths, current_path.clone());
                if i < group.items.len() - 1 {
                    // Add separator if not the last item
                }
            }
        }
    }
}
