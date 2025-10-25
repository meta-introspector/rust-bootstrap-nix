use crate::prelude::*


fn get_browser_ui_test_version_inner(
    builder: &Builder<'_>,
    npm: &Path,
    global: bool,
) -> Option<String> {
    let mut command = command(npm);
    command.arg("list").arg("--parseable").arg("--long").arg("--depth=0");
    if global {
        command.arg("--global");
    }
    let lines = command.allow_failure().run_capture(builder).stdout();
    lines
        .lines()
        .find_map(|l| l.split(':').nth(1)?.strip_prefix("browser-ui-test@"))
        .map(|v| v.to_owned())
}
