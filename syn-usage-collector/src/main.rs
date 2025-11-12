use anyhow::Result;
use walkdir::WalkDir;
use std::fs;
use syn_usage_collector::TypeUsageCollector;

fn main() -> Result<()> {
    let mut all_collected_types = TypeUsageCollector::default();

    for entry in WalkDir::new("../").into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
            println!("Processing file: {}", path.display());
            let content = fs::read_to_string(path)?;
            let ast = match syn::parse_file(&content) {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("Error parsing file {}: {}", path.display(), e);
                    continue;
                }
            };
            let collector = syn_usage_collector::analyze_file(&ast)?;
            all_collected_types.all_types.extend(collector.all_types);
        }
    }

    println!("All collected types: {:?}", all_collected_types.all_types);

    Ok(())
}
