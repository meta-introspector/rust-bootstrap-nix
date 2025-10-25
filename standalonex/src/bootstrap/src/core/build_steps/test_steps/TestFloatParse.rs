use crate::prelude::*;


            path,
            SourceType::InTree,
            &[],
        );

        run_cargo_test(
            cargo_test,
            &[],
            &[],
            crate_name,
            crate_name,
            compiler,
            bootstrap_host,
            builder,
        );

        // Run the actual parse tests.
        let mut cargo_run = tool::prepare_tool_cargo(
            builder,
            compiler,
            Mode::ToolStd,
            bootstrap_host,
            Kind::Run,
            path,
            SourceType::InTree,
            &[],
        );

        cargo_run.arg("--");
        if builder.config.args().is_empty() {
            // By default, exclude tests that take longer than ~1m.
            cargo_run.arg("--skip-huge");
        } else {
            cargo_run.args(builder.config.args());
        }

        cargo_run.into_cmd().run(builder);
    }
}
