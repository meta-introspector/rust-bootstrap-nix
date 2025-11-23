use crate::define_config;
use crate::StringOrBool;
use build_helper::prelude::*;
define_config! {
    #[derive(Deserialize)]
    struct TomlTarget {
    llvm_libunwind : Option < String > = "llvm-libunwind", split_debuginfo : Option <
    String > = "split-debuginfo", profiler : Option < StringOrBool > = "profiler",
    rpath : Option < bool > = "rpath", llvm : Option < bool > = "llvm", }
}
