use std::fs;


fn markdown_test(builder: &Builder<'_>, compiler: Compiler, markdown: &Path) -> bool {
    if let Ok(contents) = fs::read_to_string(markdown) {
        if !contents.contains("```") {
            return true;
        }
    }

    builder.verbose(|| println!("doc tests for: {}", markdown.display()));
    let mut cmd = builder.rustdoc_cmd(compiler);
    builder.add_rust_test_threads(&mut cmd);
    // allow for unstable options such as new editions
    cmd.arg("-Z");
    cmd.arg("unstable-options");
    cmd.arg("--test");
    cmd.arg(markdown);
    cmd.env("RUSTC_BOOTSTRAP", "1");

    let test_args = builder.config.test_args().join(" ");
    cmd.arg("--test-args").arg(test_args);

    cmd = cmd.delay_failure();
    if !builder.config.verbose_tests {
        cmd.run_capture(builder).is_success()
    } else {
        cmd.run(builder)
    }
}
