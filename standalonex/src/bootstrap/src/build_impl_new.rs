use crate::prelude::*;


use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs};
use std::cell::RefCell;
use std::collections::HashMap;

use build_helper::ci::gha;
use build_helper::exit;
use crate::core::config::Config;
use crate::core::config::dry_run::DryRun;
use crate::core::sanity;
use crate::core::metadata;
use crate::utils::helpers::{output, symlink_dir, dir_is_empty, exe};
use crate::utils::channel::GitInfo;
use crate::enums::DocTests;
use crate::Subcommand;
use crate::Build;

impl Build {
    /// Creates a new set of build configuration from the `flags` on the command
    /// line and the filesystem `config`.
    ///
    /// By default all build output will be placed in the current directory.
    pub fn new(mut config: Config) -> Build {
        let src = config.src.clone();
        let out = config.out.clone();

        #[cfg(unix)]
        // keep this consistent with the equivalent check in x.py:
        // https://github.com/rust-lang/rust/blob/a8a33cf27166d3eabaffc58ed3799e054af3b0c6/src/bootstrap/bootstrap.py#L796-L797
        let is_sudo = match env::var_os("SUDO_USER") {
            Some(_sudo_user) => {
                // SAFETY: getuid() system call is always successful and no return value is reserved
                // to indicate an error.
                ///
                /// For more context, see https://man7.org/linux/man-pages/man2/geteuid.2.html
                let uid = unsafe { libc::getuid() };
                uid == 0
            }
            None => false,
        };
        #[cfg(not(unix))]
        let is_sudo = false;

        let rust_info = config.rust_info.clone();
        let cargo_info = config.cargo_info.clone();
        let rust_analyzer_info = config.rust_analyzer_info.clone();
        let clippy_info = config.clippy_info.clone();
        let miri_info = config.miri_info.clone();
        let rustfmt_info = config.rustfmt_info.clone();
        let enzyme_info = config.enzyme_info.clone();
        let in_tree_llvm_info = config.in_tree_llvm_info.clone();
        let in_tree_gcc_info = config.in_tree_gcc_info.clone();

        let initial_target_libdir_str = if config.dry_run {
            "/dummy/lib/path/to/lib/".to_string()
        } else {
            output(
                Command::new(&config.initial_rustc)
                    .arg("--target")
                    .arg(config.build.rustc_target_arg())
                    .arg("--print")
                    .arg("target-libdir"),
            )
        };
        let initial_target_dir = Path::new(&initial_target_libdir_str).parent().unwrap();
        let initial_lld = initial_target_dir.join("bin").join("rust-lld");

        let initial_sysroot = if config.dry_run {
            "/dummy".to_string()
        } else {
            output(Command::new(&config.initial_rustc).arg("--print").arg("sysroot"))
        }
        .trim()
        .to_string();

        // FIXME(Zalathar): Determining this path occasionally fails locally for
        // unknown reasons, so we print some extra context to help track down why.
        let find_initial_libdir = || {
            let initial_libdir =
                initial_target_dir.parent()?.parent()?.strip_prefix(&initial_sysroot).ok()?;
            Some(initial_libdir.to_path_buf())
        };
        let Some(initial_libdir) = find_initial_libdir() else {
            panic!(
                "couldn't determine `initial_libdir`:\n- config.initial_rustc:      {rustc:?}\n- initial_target_libdir_str: {initial_target_libdir_str:?}\n- initial_target_dir:        {initial_target_dir:?}\n- initial_sysroot:           {initial_sysroot:?}\n",
                rustc = config.initial_rustc,
            );
        };

        let version = std::fs::read_to_string(src.join("src").join("version"))
            .expect("failed to read src/version");
        let version = version.trim();

        let mut bootstrap_out = std::env::current_exe ()
            .expect("could not determine path to running process")
            .parent()
            .unwrap()
            .to_path_buf();
        // Since bootstrap is hardlink to deps/bootstrap-*, Solaris can sometimes give
        // path with deps/ which is bad and needs to be avoided.
        if bootstrap_out.ends_with("deps") {
            bootstrap_out.pop();
        }
       // if !bootstrap_out.join(exe("rustc", config.build)).exists() && !cfg!(test) {
        //    // this restriction can be lifted whenever https://github.com/rust-lang/rfcs/pull/3028 is implemented
         //   panic!(
         //       "`rustc` not found in {}, run `cargo build --bins` before `cargo run`",
          //      bootstrap_out.display()
          //  )
       // }

        if rust_info.is_from_tarball() && config.description.is_none() {
            config.description = Some("built from a source tarball".to_owned());
        }

        let mut build = Build {
            initial_rustc: config.initial_rustc.clone(),
            initial_cargo: config.initial_cargo.clone(),
            initial_lld,
            initial_libdir,
            initial_sysroot: initial_sysroot.into(),
            local_rebuild: config.local_rebuild,
            fail_fast: config.cmd.fail_fast(),
            doc_tests: config.cmd.doc_tests(),
            verbosity: config.verbose,

            build: config.build,
            hosts: config.hosts.clone(),
            targets: config.targets.clone(),

            config,
            version: version.to_string(),
            src,
            out,
            bootstrap_out,

            cargo_info,
            rust_analyzer_info,
            clippy_info,
            miri_info,
            rustfmt_info,
            enzyme_info,
            in_tree_llvm_info,
            in_tree_gcc_info,
            cc: RefCell::new(HashMap::new()),
            cxx: RefCell::new(HashMap::new()),
            ar: RefCell::new(HashMap::new()),
            ranlib: RefCell::new(HashMap::new()),
            crates: HashMap::new(),
            crate_paths: HashMap::new(),
            is_sudo,
            delayed_failures: RefCell::new(Vec::new()),
            prerelease_version: std::cell::Cell::new(None),

            #[cfg(feature = "build-metrics")]
            metrics: crate::utils::metrics::BuildMetrics::init(),
        };

        // If local-rust is the same major.minor as the current version, then force a
        // local-rebuild
        let local_version_verbose =
            output(Command::new(&build.initial_rustc).arg("--version").arg("--verbose"));
        let local_release = local_version_verbose
            .lines()
            .filter_map(|x| x.strip_prefix("release:"))
            .next()
            .unwrap()
            .trim();
        if local_release.split('.').take(2).eq(version.split('.').take(2)) {
            build.verbose(|| println!("auto-detected local-rebuild {local_release}"));
            build.local_rebuild = true;
        }

        build.verbose(|| println!("finding compilers"));
        crate::utils::cc_detect::find(&build);
        // When running `setup`, the profile is about to change, so any requirements we have now may
        // be different on the next invocation. Don't check for them until the next time x.py is
        // run. This is ok because `setup` never runs any build commands, so it won't fail if commands are missing.
        //
        // Similarly, for `setup` we don't actually need submodules or cargo metadata.
        if !matches!(build.config.cmd, Subcommand::Setup { .. }) {
            build.verbose(|| println!("running sanity check"));
            crate::core::sanity::check(&mut build);

            // Make sure we update these before gathering metadata so we don't get an error about missing
            // Cargo.toml files.
            let rust_submodules = ["library/backtrace", "library/stdarch"];
            for s in rust_submodules {
                build.require_submodule(
                    s,
                    Some(
                        "The submodule is required for the standard library \ 
                         and the main Cargo workspace.",
                    ),
                );
            }
            // Now, update all existing submodules.
            build.update_existing_submodules();

            build.verbose(|| println!("learning about cargo"));
            crate::core::metadata::build(&mut build);
        }

        // Create symbolic link to use host sysroot from a consistent path (e.g., in the rust-analyzer config file).
        let build_triple = build.out.join(build.build);
        t!(fs::create_dir_all(&build_triple));
        let host = build.out.join("host");
        if host.is_symlink() {
            // Left over from a previous build; overwrite it.
            // This matters if `build.build` has changed between invocations.
            #[cfg(windows)]
            t!(fs::remove_dir(&host));
            #[cfg(not(windows))]
            t!(fs::remove_file(&host));
        }
        t!(
            symlink_dir(&build.config, &build_triple, &host),
            format!("symlink_dir({} => {})", host.display(), build_triple.display())
        );

        build
    }
}
