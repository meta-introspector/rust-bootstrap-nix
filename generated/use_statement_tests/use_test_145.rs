/// First time setup of a dev environment
///
/// These are build-and-run steps for `./x.py setup`, which allows quickly setting up the directory
/// for modifying, building, and running the compiler and library. Running arbitrary configuration
/// allows setting up things that cannot be simply captured inside the config.toml, in addition to
/// leading people away from manually editing most of the config.toml values.
use std::env::consts::EXE_SUFFIX;
