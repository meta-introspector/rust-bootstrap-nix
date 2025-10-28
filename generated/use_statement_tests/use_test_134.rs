/// C-compiler probing and detection.
///
/// This module will fill out the `cc` and `cxx` maps of `Build` by looking for
/// C and C++ compilers for each target configured. A compiler is found through
/// a number of vectors (in order of precedence)
///
/// 1. Configuration via `target.$target.cc` in `config.toml`.
/// 2. Configuration via `target.$target.android-ndk` in `config.toml`, if
///    applicable
/// 3. Special logic to probe on OpenBSD
/// 4. The `CC_$target` environment variable.
/// 5. The `CC` environment variable.
/// 6. "cc"
///
/// Some of this logic is implemented here, but much of it is farmed out to the
/// `cc` crate itself, so we end up having the same fallbacks as there.
/// Similar logic is then used to find a C++ compiler, just some s/cc/c++/ is
/// used.
///
/// It is intended that after this module has run no C/C++ compiler will
/// ever be probed for. Instead the compilers found here will be used for
/// everything.
use std::collections::HashSet;
