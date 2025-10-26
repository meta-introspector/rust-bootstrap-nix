use crate::prelude::*;
pub fn get_flake_input(flake_path_str: &str, input_name: &str) -> Result<String> {
    let path_template = "path:{}";
    let path_expr = "(builtins.getFlake {}).inputs.{}.outPath";
    let composed_path = compose_path::compose_path(path_expr, path_template);
    let expr = format_new::format_new(&composed_path, flake_path_str, input_name);
    let stdout = run_nix_eval(&expr)?;
    debug!("Nix command stdout: {}", stdout);
    Ok(stdout)
}
