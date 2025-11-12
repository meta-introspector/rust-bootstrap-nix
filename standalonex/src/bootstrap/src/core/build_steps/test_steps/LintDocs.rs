use crate::prelude::*;


            compiler,
            Mode::ToolBootstrap,
            bootstrap_host,
            Kind::Test,
            "src/tools/rust-installer",
            SourceType::InTree,
            &[],
        );

        let _guard = builder.msg(
            Kind::Test,
            compiler.stage,
            "rust-installer",
            bootstrap_host,
            bootstrap_host,
        );
        run_cargo_test(cargo, &[], &[], "installer", None, compiler, bootstrap_host, builder);

        // We currently don't support running the test.sh script outside linux(?) environments.
        // Eventually this should likely migrate to #[test]s in rust-installer proper rather than a
        // set of scripts, which will likely allow dropping this if.
        if bootstrap_host != "x86_64-unknown-linux-gnu" {
            return;
        }

        let mut cmd = command(builder.src.join("src/tools/rust-installer/test.sh"));
        let tmpdir = testdir(builder, compiler.host).join("rust-installer");
        let _ = std::fs::remove_dir_all(&tmpdir);
        let _ = std::fs::create_dir_all(&tmpdir);
        cmd.current_dir(&tmpdir);
        cmd.env("CARGO_TARGET_DIR", tmpdir.join("cargo-target"));
        cmd.env("CARGO", &builder.initial_cargo);
        cmd.env("RUSTC", &builder.initial_rustc);
        cmd.env("TMP_DIR", &tmpdir);
        cmd.delay_failure().run(builder);
