
/// Each path in this list is considered "allowed" in the `download-rustc="if-unchanged"` logic.
/// This means they can be modified and changes to these paths should never trigger a compiler build
/// when "if-unchanged" is set.
///
/// NOTE: Paths must have the ":!" prefix to tell git to ignore changes in those paths during
/// the diff check.
///
/// WARNING: Be cautious when adding paths to this list. If a path that influences the compiler build
/// is added here, it will cause bootstrap to skip necessary rebuilds, which may lead to risky results.
/// For example, "src/bootstrap" should never be included in this list as it plays a crucial role in the
/// final output/compiler, which can be significantly affected by changes made to the bootstrap sources.

//pub(crate) const RUSTC_IF_UNCHANGED_ALLOWED_PATHS: &[&str] = &[
//#    ":!src/tools",
//   ":!tests",
//    ":!triagebot.toml",
//];

macro_rules! check_ci_llvm {
    ($name:expr) => {
        assert!(
            $name.is_none(),
            "setting {} is incompatible with download-ci-llvm.",
            stringify!($name).replace("_", "-")
        );
    };
}

