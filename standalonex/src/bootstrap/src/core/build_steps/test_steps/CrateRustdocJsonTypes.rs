            &["rustdoc-json-types".to_string()],
            "rustdoc-json-types",
            "rustdoc-json-types",
            compiler,
            target,
            builder,
        );
    }
}

/// Some test suites are run inside emulators or on remote devices, and most
/// of our test binaries are linked dynamically which means we need to ship
/// the standard library and such to the emulator ahead of time. This step
/// represents this and is a dependency of all test suites.
///
/// Most of the time this is a no-op. For some steps such as shipping data to
/// QEMU we have to build our own tools so we've got conditional dependencies
/// on those programs as well. Note that the remote test client is built for
/// the build target (us) and the server is built for the target.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RemoteCopyLibs {
    compiler: Compiler,
    target: TargetSelection,
}

impl Step for RemoteCopyLibs {
    type Output = ();

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.never()
    }

    fn run(self, builder: &Builder<'_>) {
        let compiler = self.compiler;
        let target = self.target;
        if !builder.remote_tested(target) {
            return;
        }

        builder.ensure(compile::Std::new(compiler, target));

        builder.info(&format!("REMOTE copy libs to emulator ({target})"));

        let server = builder.ensure(tool::RemoteTestServer { compiler, target });

        // Spawn the emulator and wait for it to come online
        let tool = builder.tool_exe(Tool::RemoteTestClient);
        let mut cmd = command(&tool);
        cmd.arg("spawn-emulator").arg(target.triple).arg(&server).arg(builder.tempdir());
        if let Some(rootfs) = builder.qemu_rootfs(target) {
            cmd.arg(rootfs);
        }
        cmd.run(builder);

        // Push all our dylibs to the emulator
        for f in t!(builder.sysroot_target_libdir(compiler, target).read_dir()) {
            let f = t!(f);
            if helpers::is_dylib(&f.path()) {
                command(&tool).arg("push").arg(f.path()).run(builder);
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Distcheck;

impl Step for Distcheck {
    type Output = ();

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
