use std::collections::HashMap;
use std::path::Path;
use anyhow::{Context, Result};
use cargo_metadata::MetadataCommand;
use split_expanded_lib::ResolvedDependency;



#[derive(Debug)]
pub struct SymbolMap {
    pub map: HashMap<String, ResolvedDependency>,
}

impl SymbolMap {
    pub fn new() -> Self {
        SymbolMap {
            map: HashMap::new(),
        }
    }

    pub fn populate_from_cargo_metadata(&mut self, workspace_path: &Path) -> Result<()> {
        let metadata = MetadataCommand::new()
            .current_dir(workspace_path)
            .exec()
            .context("Failed to run cargo metadata")?;

        for package in metadata.packages {
            let crate_name = package.name;
            // For now, a simple heuristic: assume all top-level items in a package belong to that crate.
            // This will need to be refined with actual AST analysis later.
            // We'll just add the crate name itself as a resolved dependency for now.
            self.map.insert(crate_name.clone(), ResolvedDependency {
                id: crate_name.clone(),
                dependency_type: "crate".to_string(),
                crate_name: crate_name.clone(),
                module_path: crate_name.clone(), // Placeholder
                usage_count: 0,
            });

            // TODO: Parse source files of each package to extract actual symbols and their module paths.
        }
        Ok(())
    }

    pub fn resolve(&self, id: &str) -> Option<ResolvedDependency> {
        self.map.get(id).cloned()
    }

    pub fn add_declaration(&mut self, id: String, dependency_type: String, crate_name: String, module_path: String) {
        self.map.entry(id.clone()).or_insert_with(|| ResolvedDependency {
            id,
            dependency_type,
            crate_name,
            module_path,
            usage_count: 0,
        });
    }

    pub fn resolve_and_increment_usage(&mut self, id: String, dependency_type: String, crate_name: String, module_path: String) -> ResolvedDependency {
        let entry = self.map.entry(id.clone()).or_insert_with(|| ResolvedDependency {
            id,
            dependency_type,
            crate_name,
            module_path,
            usage_count: 0,
        });
        entry.usage_count += 1;
        entry.clone()
    }
}
