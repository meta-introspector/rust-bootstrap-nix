use crate::prelude::*;


use log::info;
use log::debug;
use crate::utils::nix_eval_utils::run_nix_eval;
use crate::utils::get_all_rustc_paths_from_nix_store::get_all_rustc_paths_from_nix_store;
use anyhow::Result;
pub fn find_nix_package_store_path(package_name: &str, version: Option<&str>) -> Result<Option<String>> {
    info!("Searching for Nix package: {}", package_name);
    if let Some(v) = version {
        info!("  Version: {}", v);
    }

    if package_name == "rustc" {
        let rustc_versions = get_all_rustc_paths_from_nix_store()?;
        if rustc_versions.is_empty() {
            return Ok(None);
        }

        if let Some(v) = version {
            // If a specific version is requested, find it
            for (path, found_version) in rustc_versions {
                if found_version == v {
                    return Ok(Some(path));
                }
            }
            return Ok(None); // Version not found
        } else {
            // If no specific version is requested, and we're in find_nix_package_store_path,
            // it's ambiguous. The --list-rustc-versions flag handles listing all.
            // So, return None here to avoid ambiguity.
            return Ok(None);
        }
    }

    let expr = format!("with import <nixpkgs> {{}}; pkgs.{}.outPath", package_name);

    let stdout = run_nix_eval(&expr);

    match stdout {
        Ok(path) => Ok(Some(path)),
        Err(e) => {
            debug!("Nix eval failed for package '{}': {}", package_name, e);
            Ok(None)
        }
    }
}
