use crate::prelude::*;


    }

    fn run(self, builder: &Builder<'_>) {
        let compiler = self.common.compiler;
        let target = self.common.target;

        builder.ensure(compile::Std::new(compiler, target));

        // If we're not doing a full bootstrap but we're testing a stage2
        // version of libstd, then what we're actually testing is the libstd
        // produced in stage1. Reflect that here by updating the compiler that
        // we're working with automatically.
        let compiler = builder.compiler_for(compiler.stage, compiler.host, target);

        let build_cargo = || {
            let mut cargo = builder::Cargo::new(
                builder,
                compiler,
                Mode::Codegen, // Must be codegen to ensure dlopen on compiled dylibs works
                SourceType::InTree,
                target,
                Kind::Run,
            );

            cargo.current_dir(&builder.src.join("compiler/rustc_codegen_cranelift"));
            cargo
                .arg("--manifest-path")
                .arg(builder.src.join("compiler/rustc_codegen_cranelift/build_system/Cargo.toml"));
            compile::rustc_cargo_env(builder, &mut cargo, target, compiler.stage);

            // Avoid incremental cache issues when changing rustc
            cargo.env("CARGO_BUILD_INCREMENTAL", "false");

            cargo
        };

        builder.info(&format!(
            "{} cranelift stage{} ({} -> {})",
            Kind::Test.description(),
            compiler.stage,
            &compiler.host,
            target
        ));
        let _time = helpers::timeit(builder);

        // FIXME handle vendoring for source tarballs before removing the --skip-test below
        let download_dir = builder.out.join("cg_clif_download");

        // FIXME: Uncomment the `prepare` command below once vendoring is implemented.
        /*
        let mut prepare_cargo = build_cargo();
        prepare_cargo.arg("--").arg("prepare").arg("--download-dir").arg(&download_dir);
        #[allow(deprecated)]
        builder.config.try_run(&mut prepare_cargo.into()).unwrap();
        */

        let mut cargo = build_cargo();
        cargo
            .arg("--")
            .arg("test")
            .arg("--download-dir")
            .arg(&download_dir)
            .arg("--out-dir")
            .arg(builder.stage_out(compiler, Mode::ToolRustc).join("cg_clif"))
            .arg("--no-unstable-features")
            .arg("--use-backend")
            .arg("cranelift")
            // Avoid having to vendor the standard library dependencies
            .arg("--sysroot")
            .arg("llvm")
            // These tests depend on crates that are not yet vendored
            // FIXME remove once vendoring is handled
            .arg("--skip-test")
            .arg("testsuite.extended_sysroot");
        cargo.args(builder.config.test_args());

        cargo.into_cmd().run(builder);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CodegenGCC {
    pub common: common_test_fields::CommonTestFields,
}

impl Step for CodegenGCC {
    type Output = ();
    const DEFAULT: bool = true;
    const ONLY_HOSTS: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.paths(&["compiler/rustc_codegen_gcc"])
    }

    fn make_run(run: RunConfig<'_>) {
        let builder = run.builder;
        let host = run.build_triple();
        let compiler = run.builder.compiler_for(run.builder.top_stage, host, host);

        if builder.doc_tests == DocTests::Only {
            return;
        }

        if builder.download_rustc() {
            builder.info("CI rustc uses the default codegen backend. skipping");
            return;
        }

        let triple = run.target.triple;
        let target_supported =
            if triple.contains("linux") { triple.contains("x86_64") } else { false };
        if !target_supported {
            builder.info("target not supported by rustc_codegen_gcc. skipping");
            return;
        }

        if builder.remote_tested(run.target) {
            builder.info("remote testing is not supported by rustc_codegen_gcc. skipping");
            return;
        }

        if !builder.config.codegen_backends(run.target).contains(&"gcc".to_owned()) {
            builder.info("gcc not in rust.codegen-backends. skipping");
            return;
        }

        builder.ensure(CodegenGCC {
            common: common_test_fields::CommonTestFields {
                stage: run.builder.top_stage,
                host: run.build_triple(),
                compiler,
