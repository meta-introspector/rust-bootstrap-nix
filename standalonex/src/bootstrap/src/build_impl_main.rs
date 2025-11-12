use build_helper::ci::gha;
use build_helper::exit;
use crate::core::builder;
use crate::core::config::dry_run::DryRun;
use crate::Subcommand;
use crate::Build;

impl Build {
    /// Executes the entire build, as configured by the flags and configuration.
    pub fn build(&mut self) {
        unsafe {
            crate::utils::job::setup(self);
        }

        // Download rustfmt early so that it can be used in rust-analyzer configs.
        let _ = &builder::Builder::new(self).initial_rustfmt();

        // hardcoded subcommands
        match &self.config.cmd {
            Subcommand::Format { check, all } => {
                return core::build_steps::format::format(
                    &builder::Builder::new(self),
                    *check,
                    all,
                    &self.config.paths,
                );
            }
            Subcommand::Suggest { run } => {
                return core::build_steps::suggest::suggest(&builder::Builder::new(self), run);
            }
            Subcommand::Perf { .. } => {
                return core::build_steps::perf::perf(&builder::Builder::new(self));
            }
            _ => (),
        }

        if !self.config.dry_run {
            {
                // We first do a dry-run. This is a sanity-check to ensure that
                // steps don't do anything expensive in the dry-run.
                self.config.dry_run = DryRun::SelfCheck;
                let builder = builder::Builder::new(self);
                builder.execute_cli();
            }
            self.config.dry_run = DryRun::Disabled;
            let builder = builder::Builder::new(self);
            builder.execute_cli();
        } else {
            let builder = builder::Builder::new(self);
            builder.execute_cli();
        }

        // Check for postponed failures from `test --no-fail-fast`.
        let failures = self.delayed_failures.borrow();
        if failures.len() > 0 {
            eprintln!("\n{} command(s) did not execute successfully:\n", failures.len());
            for failure in failures.iter() {
                eprintln!("  - {}\n", failure);
            }
            exit!(1);
        }

        #[cfg(feature = "build-metrics")]
        self.metrics.persist(self);
    }

    #[track_caller]
    fn group(&self, msg: &str) -> Option<gha::Group> {
        match self.config.dry_run {
            DryRun::SelfCheck => None,
            DryRun::Disabled | DryRun::UserSelected => Some(gha::group(msg)),
        }
    }

    /// Return a `Group` guard for a [`Step`] that is built for each `--stage`.
    ///
    /// [`Step`]: crate::core::builder::Step
    #[must_use = "Groups should not be dropped until the Step finishes running"]
    #[track_caller]
    fn msg(
        &self,
        action: impl Into<builder::Kind>,
        stage: u32,
        what: impl Display,
        host: impl Into<Option<TargetSelection>>,
        target: impl Into<Option<TargetSelection>>,
    ) -> Option<gha::Group> {
        let action = action.into().description();
        let msg = |fmt| format!("{action} stage{stage} {what}{fmt}");
        let msg = if let Some(target) = target.into() {
            let host = host.into().unwrap();
            if host == target {
                msg(format_args!(" ({target})"))
            } else {
                msg(format_args!(" ({host} -> {target})"))
            }
        } else {
            msg(format_args!(""))
        };
        self.group(&msg)
    }

    /// Return a `Group` guard for a [`Step`] that is only built once and isn't affected by `--stage`.
    ///
    /// [`Step`]: crate::core::builder::Step
    #[must_use = "Groups should not be dropped until the Step finishes running"]
    #[track_caller]
    fn msg_unstaged(
        &self,
        action: impl Into<builder::Kind>,
        what: impl Display,
        target: TargetSelection,
    ) -> Option<gha::Group> {
        let action = action.into().description();
        let msg = format!("{action} {what} for {target}");
        self.group(&msg)
    }

    #[must_use = "Groups should not be dropped until the Step finishes running"]
    #[track_caller]
    fn msg_sysroot_tool(
        &self,
        action: impl Into<builder::Kind>,
        stage: u32,
        what: impl Display,
        host: TargetSelection,
        target: TargetSelection,
    ) -> Option<gha::Group> {
        let action = action.into().description();
        let msg = |fmt| format!("{action} {what} {fmt}");
        let msg = if host == target {
            msg(format_args!("(stage{stage} -> stage{}, {target})", stage + 1))
        } else {
            msg(format_args!("(stage{stage}:{host} -> stage{}:{target})", stage + 1))
        };
        self.group(&msg)
    }

    #[must_use = "Groups should not be dropped until the Step finishes running"]
    #[track_caller]
    fn msg_clippy(
        &self,
        what: impl Display,
        target: impl Into<Option<TargetSelection>>,
    ) -> Option<gha::Group> {
        self.msg(builder::Kind::Clippy, self.config.stage, what, self.config.build, target)
    }

    #[must_use = "Groups should not be dropped until the Step finishes running"]
    #[track_caller]
    fn msg_check(
        &self,
        what: impl Display,
        target: impl Into<Option<TargetSelection>>,
    ) -> Option<gha::Group> {
        self.msg(builder::Kind::Check, self.config.stage, what, self.config.build, target)
    }

    #[must_use = "Groups should not be dropped until the Step finishes running"]
    #[track_caller]
    fn msg_doc(
        &self,
        compiler: Compiler,
        what: impl Display,
        target: impl Into<Option<TargetSelection>> + Copy,
    ) -> Option<gha::Group> {
        self.msg(builder::Kind::Doc, compiler.stage, what, compiler.host, target.into())
    }

    #[must_use = "Groups should not be dropped until the Step finishes running"]
    #[track_caller]
    fn msg_build(
        &self,
        compiler: Compiler,
        what: impl Display,
        target: impl Into<Option<TargetSelection>>,
    ) -> Option<gha::Group> {
        self.msg(builder::Kind::Build, compiler.stage, what, compiler.host, target)
    }
}
