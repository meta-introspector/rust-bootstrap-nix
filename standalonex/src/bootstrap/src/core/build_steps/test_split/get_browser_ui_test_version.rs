fn get_browser_ui_test_version(builder: &Builder<'_>, npm: &Path) -> Option<String> {
    get_browser_ui_test_version_inner(builder, npm, false)
        .or_else(|| get_browser_ui_test_version_inner(builder, npm, true))
}
