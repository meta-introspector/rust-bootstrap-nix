/// Compilation of native dependencies like GCC.
///
/// Native projects like GCC unfortunately aren't suited just yet for
/// compilation in build scripts that Cargo has. This is because the
/// compilation takes a *very* long time but also because we don't want to
/// compile GCC 3 times as part of a normal bootstrap (we want it cached).
///
/// GCC and compiler-rt are essentially just wired up to everything else to
/// ensure that they're always in place if needed.
use std::fs;
