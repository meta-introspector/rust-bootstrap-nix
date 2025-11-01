use build_helper::prelude::*;
define_config! {
    struct TomlTarget {
    llvm_libunwind : Option < String > = "llvm-libunwind", split_debuginfo : Option <
    String > = "split-debuginfo", profiler : Option < StringOrBool > = "profiler",
    rpath : Option < bool > = "rpath", llvm : Option < bool > = "llvm", }
}
