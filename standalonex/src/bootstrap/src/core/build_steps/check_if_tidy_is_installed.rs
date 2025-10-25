use crate::prelude::*;


use crate::core::builder::Builder;
use crate::utils::exec::command;

pub fn check_if_tidy_is_installed(builder: &Builder<'_>) -> bool {
    command("tidy").allow_failure().arg("--version").run_capture_stdout(builder).is_success()
}
