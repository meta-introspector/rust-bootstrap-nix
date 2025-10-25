use crate::prelude::*;


        if !target.is_msvc() {
            if let Some(ar) = builder.ar(target) {
                cfg.archiver(ar);
            }
            cfg.compiler(builder.cc(target));
        }
        cfg.cargo_metadata(false)
            .out_dir(&dst)
            .target(&target.triple)
            .host(&builder.config.build.triple)
            .opt_level(0)
            .warnings(false)
            .debug(false)
            .file(builder.src.join("tests/auxiliary/rust_test_helpers.c"))
            .compile("rust_test_helpers");
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CodegenCranelift {
    pub common: common_test_fields::CommonTestFields,
}

impl Step for CodegenCranelift {
    type Output = ();
    const DEFAULT: bool = true;
    const ONLY_HOSTS: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.paths(&["compiler/rustc_codegen_cranelift"])
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

        if !target_supports_cranelift_backend(run.target) {
            builder.info("target not supported by rustc_codegen_cranelift. skipping");
            return;
        }

        if builder.remote_tested(run.target) {
            builder.info("remote testing is not supported by rustc_codegen_cranelift. skipping");
            return;
        }

        if !builder.config.codegen_backends(run.target).contains(&"cranelift".to_owned()) {
            builder.info("cranelift not in rust.codegen-backends. skipping");
            return;
        }

        builder.ensure(CodegenCranelift {
            common: common_test_fields::CommonTestFields {
                stage: run.builder.top_stage,
                host: run.build_triple(),
                compiler,
                target: run.target,
            },
        });
