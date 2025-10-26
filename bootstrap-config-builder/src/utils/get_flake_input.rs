


use anyhow::{Result};
use super::compose_path; // Import from sibling module
use super::format_new;   // Import from sibling module
use log::debug; // Import only debug macro
//use crate::utils::nix_eval_utils; // Import the new module
use crate::utils::nix_eval_utils::run_nix_eval;
pub fn get_flake_input(flake_path_str: &str, input_name: &str) -> Result<String> {
    let path_template = "path:{}";
    let path_expr = "(builtins.getFlake {}).inputs.{}.outPath";

    let composed_path = compose_path::compose_path(path_expr, path_template);
    let expr = format_new::format_new(&composed_path, flake_path_str, input_name);

    let stdout = run_nix_eval(&expr)?;
    debug!("Nix command stdout: {}", stdout);
    Ok(stdout)
}
