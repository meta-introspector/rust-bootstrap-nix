/// Compilation of native dependencies like LLVM.
///
/// Native projects like LLVM unfortunately aren't suited just yet for
/// compilation in build scripts that Cargo has. This is because the
/// compilation takes a *very* long time but also because we don't want to
/// compile LLVM 3 times as part of a normal bootstrap (we want it cached).
///
/// LLVM and compiler-rt are essentially just wired up to everything else to
/// ensure that they're always in place if needed.
use std::env;
