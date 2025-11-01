use build_helper::prelude::*;
define_config! {
    #[doc = " TOML representation of how the LLVM build is configured."] struct Llvm {
    optimize : Option < bool > = "optimize", thin_lto : Option < bool > = "thin-lto",
    release_debuginfo : Option < bool > = "release-debuginfo", assertions : Option < bool
    > = "assertions", tests : Option < bool > = "tests", enzyme : Option < bool > =
    "enzyme", plugins : Option < bool > = "plugins", ccache : Option < StringOrBool > =
    "ccache", static_libstdcpp : Option < bool > = "static-libstdcpp", libzstd : Option <
    bool > = "libzstd", ninja : Option < bool > = "ninja", targets : Option < String > =
    "targets", experimental_targets : Option < String > = "experimental-targets",
    link_jobs : Option < u32 > = "link-jobs", link_shared : Option < bool > =
    "link-shared", version_suffix : Option < String > = "version-suffix", clang_cl :
    Option < String > = "clang-cl", cflags : Option < String > = "cflags", cxxflags :
    Option < String > = "cxxflags", ldflags : Option < String > = "ldflags", use_libcxx :
    Option < bool > = "use-libcxx", use_linker : Option < String > = "use-linker",
    allow_old_toolchain : Option < bool > = "allow-old-toolchain", offload : Option <
    bool > = "offload", polly : Option < bool > = "polly", clang : Option < bool > =
    "clang", enable_warnings : Option < bool > = "enable-warnings", download_ci_llvm :
    Option < StringOrBool > = "download-ci-llvm", build_config : Option < HashMap <
    String, String >> = "build-config", enable_projects : Option < String > =
    "enable-projects", }
}
