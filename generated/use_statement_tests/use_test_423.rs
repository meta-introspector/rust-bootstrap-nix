/// This module renders the JSON output of libtest into a human-readable form, trying to be as
/// similar to libtest's native output as possible.
///
/// This is needed because we need to use libtest in JSON mode to extract granular information
/// about the executed tests. Doing so suppresses the human-readable output, and (compared to Cargo
/// and rustc) libtest doesn't include the rendered human-readable output as a JSON field. We had
/// to reimplement all the rendering logic in this module because of that.
use std::io::{BufRead, BufReader, Read, Write};
