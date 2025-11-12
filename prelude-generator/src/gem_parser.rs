use serde::Deserialize;
use std::collections::HashMap;
use anyhow::Result;

#[derive(Debug, Deserialize)]
pub struct GemConfig {
    pub gem: Vec<GemEntry>,
}

#[derive(Debug, Deserialize)]
pub struct GemEntry {
    pub name: String,
    pub crate_name: String, // Renamed from 'crate' to 'crate_name' to avoid keyword conflict
    pub identifiers: Vec<String>,
}

impl Default for GemConfig {
    fn default() -> Self {
        GemConfig {
            gem: Vec::new(),
        }
    }
}

impl GemConfig {
    pub fn load_from_file(path: &std::path::Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: GemConfig = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn get_identifier_to_gem_map(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        for gem_entry in &self.gem {
            for identifier in &gem_entry.identifiers {
                map.insert(identifier.clone(), gem_entry.name.clone());
            }
        }
        map
    }
}
