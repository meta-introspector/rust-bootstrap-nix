pub fn compose_path(path_expr: &str, path_template: &str) -> String {
    path_expr.replacen("{}", &format!("{}", path_template), 1)
}