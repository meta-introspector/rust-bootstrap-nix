pub fn extract_rustc_version_from_path(path: &str) -> String {
    // Example path: /nix/store/0x7s63gjcvybs6fgdq9p6z5l8svcxaav-nix-on-droid-path/bin/rustc
    // We want to extract "nix-on-droid-path" or similar version info.
    // A more robust solution might involve `rustc --version` but for now,
    // let's try to parse the path.
    let parts: Vec<&str> = path.split('/').collect();
    if parts.len() >= 4 {
        // Expecting something like "hash-name-version"
        let store_dir = parts[3];
        if let Some(dash_index) = store_dir.find('-') {
            // Try to find the second dash to get the name-version part
            if let Some(second_dash_index) = store_dir[dash_index + 1..].find('-') {
                return store_dir[dash_index + 1 + second_dash_index..].to_string();
            }
        }
        // Fallback if parsing fails
        store_dir.to_string()
    } else {
        "unknown".to_string()
    }
}
