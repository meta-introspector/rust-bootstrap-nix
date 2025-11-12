use bootstrap::Builder;

pub fn check_if_tidy_is_installed(builder: &Builder<'_>) -> bool {
    builder.config.tidy
        && builder.config.tools.as_ref().map_or(false, |tools| tools.contains("tidy"))
}
