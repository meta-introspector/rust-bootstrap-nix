        }

        if let Some(initial_cargo) = builder.config.initial_cargo.to_str() {
            cmd.arg("--initial-cargo").arg(initial_cargo);
        }

        cmd.arg("--jobs").arg(builder.jobs().to_string());

        cmd.env("RUSTDOC", builder.rustdoc(self.common.compiler))
            .env("RUSTC", builder.rustc(self.common.compiler));

        add_rustdoc_cargo_linker_args(&mut cmd, builder, self.common.compiler.host, LldThreads::No);

        for path in &builder.paths {
            if let Some(p) = helpers::is_valid_test_suite_arg(p, "tests/rustdoc-gui", builder) {
                if !p.ends_with(".goml") {
                    eprintln!("A non-goml file was given: `{}`", path.display());
                    panic!("Cannot run rustdoc-gui tests");
                }
                if let Some(name) = path.file_name().and_then(|f| f.to_str()) {
                    cmd.arg("--goml-file").arg(name);
                }
            }
        }

        for test_arg in builder.config.test_args() {
            cmd.arg("--test-arg").arg(test_arg);
        }

        if let Some(ref nodejs) = builder.config.nodejs {
            cmd.arg("--nodejs").arg(nodejs);
        }

        if let Some(ref npm) = builder.config.npm {
            cmd.arg("--npm").arg(npm);
        }

        let _time = helpers::timeit(builder);
        let _guard = builder.msg_sysroot_tool(
            Kind::Test,
            self.common.compiler.stage,
            "rustdoc-gui",
            self.common.compiler.host,
            self.common.target,
        );
        try_run_tests(builder, &mut cmd, true);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tidy;

impl Step for Tidy {
    type Output = ();
    const DEFAULT: bool = true;
    const ONLY_HOSTS: bool = true;

    /// Runs the `tidy` tool.
    ///
    /// This tool in `src/tools` checks up on various bits and pieces of style and
    /// otherwise just implements a few lint-like checks that are specific to the
    /// compiler itself.
    ///
    /// Once tidy passes, this step also runs `fmt --check` if tests are being run
    /// for the `dev` or `nightly` channels.
    fn run(self, builder: &Builder<'_>) {
        let mut cmd = builder.tool_cmd(Tool::Tidy);
        cmd.arg(&builder.src);
        cmd.arg(&builder.initial_cargo);
        cmd.arg(&builder.out);
        // Tidy is heavily IO constrained. Still respect `-j`, but use a higher limit if `jobs` hasn't been configured.
        let jobs = builder.config.jobs.unwrap_or_else(|| {
            8 * std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get) as u32
        });
        cmd.arg(jobs.to_string());
        if builder.is_verbose() {
            cmd.arg("--verbose");
        }
        if builder.config.cmd.bless() {
            cmd.arg("--bless");
        }
        if let Some(s) = builder.config.cmd.extra_checks() {
            cmd.arg(format!("--extra-checks={s}"));
        }
        let mut args = std::env::args_os();
        if args.any(|arg| arg == OsStr::new("--")) {
            cmd.arg("--");
            cmd.args(args);
        }

        if builder.config.channel == "dev" || builder.config.channel == "nightly" {
            builder.info("fmt check");
            if builder.initial_rustfmt().is_none() {
                let inferred_rustfmt_dir = builder.initial_rustc.parent().unwrap();
                eprintln!(
                    "\
ERROR: no `rustfmt` binary found in {PATH}
INFO: `rust.channel` is currently set to \"{CHAN}\"
