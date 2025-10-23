use crate::Mode;

pub enum DocTests {
    /// Run normal tests and doc tests (default).
    Yes,
    /// Do not run any doc tests.
    No,
    /// Only run doc tests.
    Only,
}

pub enum GitRepo {
    Rustc,
    Llvm,
}

pub enum CLang {
    C,
    Cxx,
}

/// The various "modes" of invoking Cargo.
///
/// These entries currently correspond to the various output directories of the
/// build system, with each mod generating output in a different directory.
#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Mode {
    /// Build the standard library, placing output in the "stageN-std" directory.
    Std,

    /// Build librustc, and compiler libraries, placing output in the "stageN-rustc" directory.
    Rustc,

    /// Build a codegen backend for rustc, placing the output in the "stageN-codegen" directory.
    Codegen,

    /// Build a tool, placing output in the "stage0-bootstrap-tools"
    /// directory. This is for miscellaneous sets of tools that are built
    /// using the bootstrap stage0 compiler in its entirety (target libraries
    /// and all). Typically these tools compile with stable Rust.
    ///
    /// Only works for stage 0.
    ToolBootstrap,

    /// Build a tool which uses the locally built std, placing output in the
    /// "stageN-tools" directory. Its usage is quite rare, mainly used by
    /// compiletest which needs libtest.
    ToolStd,

    /// Build a tool which uses the locally built rustc and the target std,
    /// placing the output in the "stageN-tools" directory. This is used for
    /// anything that needs a fully functional rustc, such as rustdoc, clippy,
    /// cargo, rls, rustfmt, miri, etc.
    ToolRustc,
}

impl Mode {
    pub fn is_tool(&self) -> bool {
        matches!(self, Mode::ToolBootstrap | Mode::ToolRustc | Mode::ToolStd)
    }

    pub fn must_support_dlopen(&self) -> bool {
        matches!(self, Mode::Std | Mode::Codegen)
    }
}
