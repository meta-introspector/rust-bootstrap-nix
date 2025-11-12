use crate::prelude::*;


use bootstrap::Builder;
use std::path::Path;

pub fn get_browser_ui_test_version(builder: &Builder<'_>, npm: &Path) -> Option<String> {
    builder.info.browser_ui_test
        .as_ref()
        .and_then(|s| {
            if s == "auto" {
                // Assuming get_browser_ui_test_version_inner will also be moved
                // For now, I'll keep it as is and fix it later.
                get_browser_ui_test_version_inner(builder, npm)
            } else {
                Some(s.to_string())
            }
        })
}
