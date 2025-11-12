use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Default, Deserialize)]
pub struct AppConfig {
    pub stage: Option<String>,
    pub target: Option<String>,
    pub project_root: Option<PathBuf>,
    pub system: Option<String>,
    pub output: Option<PathBuf>,
    pub rust_bootstrap_nix_flake_ref: Option<String>,
    pub rust_src_flake_ref: Option<String>,
    pub nixpkgs_path: Option<PathBuf>,
    pub rust_overlay_path: Option<PathBuf>,
    pub rust_bootstrap_nix_path: Option<PathBuf>,
    pub configuration_nix_path: Option<PathBuf>,
    pub rust_src_flake_path: Option<PathBuf>,
    pub dry_run: Option<bool>,
    pub rustc_path: Option<PathBuf>,
    pub cargo_path: Option<PathBuf>,
    pub rust_channel: Option<String>,
    pub rust_download_rustc: Option<bool>,
    pub rust_parallel_compiler: Option<bool>,
    pub rust_llvm_tools: Option<bool>,
    pub rust_debuginfo_level: Option<u8>,
    pub patch_binaries_for_nix: Option<bool>,
    pub vendor: Option<bool>,
    pub build_dir: Option<PathBuf>,
    pub build_jobs: Option<u32>,
    pub home_dir: Option<PathBuf>,
    pub cargo_home_dir: Option<PathBuf>,
    pub install_prefix: Option<PathBuf>,
    pub install_sysconfdir: Option<PathBuf>,
    pub dist_sign_folder: Option<PathBuf>,
    pub dist_upload_addr: Option<String>,
    pub llvm_download_ci_llvm: Option<bool>,
    pub llvm_ninja: Option<bool>,
    pub change_id: Option<String>,
}

impl AppConfig {
    pub fn merge_with_args(&mut self, args: &crate::args::Args) {
        if let Some(stage) = args.stage.clone() { self.stage = Some(stage); }
        if let Some(target) = args.target.clone() { self.target = Some(target); }
        if let Some(project_root) = args.project_root.clone() { self.project_root = Some(project_root); }
        if let Some(system) = args.system.clone() { self.system = Some(system); }
        if let Some(output) = args.output.clone() { self.output = Some(output); }
        if let Some(rust_bootstrap_nix_flake_ref) = args.rust_bootstrap_nix_flake_ref.clone() { self.rust_bootstrap_nix_flake_ref = Some(rust_bootstrap_nix_flake_ref); }
        if let Some(rust_src_flake_ref) = args.rust_src_flake_ref.clone() { self.rust_src_flake_ref = Some(rust_src_flake_ref); }
        if let Some(nixpkgs_path) = args.nixpkgs_path.clone() { self.nixpkgs_path = Some(nixpkgs_path); }
        if let Some(rust_overlay_path) = args.rust_overlay_path.clone() { self.rust_overlay_path = Some(rust_overlay_path); }
        if let Some(rust_bootstrap_nix_path) = args.rust_bootstrap_nix_path.clone() { self.rust_bootstrap_nix_path = Some(rust_bootstrap_nix_path); }
        if let Some(configuration_nix_path) = args.configuration_nix_path.clone() { self.configuration_nix_path = Some(configuration_nix_path); }
        if let Some(rust_src_flake_path) = args.rust_src_flake_path.clone() { self.rust_src_flake_path = Some(rust_src_flake_path); }
        self.dry_run = Some(args.dry_run);
        if let Some(rustc_path) = args.rustc_path.clone() { self.rustc_path = Some(rustc_path); }
        if let Some(cargo_path) = args.cargo_path.clone() { self.cargo_path = Some(cargo_path); }
        if let Some(rust_channel) = args.rust_channel.clone() { self.rust_channel = Some(rust_channel); }
        if let Some(rust_download_rustc) = args.rust_download_rustc { self.rust_download_rustc = Some(rust_download_rustc); }
        if let Some(rust_parallel_compiler) = args.rust_parallel_compiler { self.rust_parallel_compiler = Some(rust_parallel_compiler); }
        if let Some(rust_llvm_tools) = args.rust_llvm_tools { self.rust_llvm_tools = Some(rust_llvm_tools); }
        if let Some(rust_debuginfo_level) = args.rust_debuginfo_level { self.rust_debuginfo_level = Some(rust_debuginfo_level); }
        if let Some(patch_binaries_for_nix) = args.patch_binaries_for_nix { self.patch_binaries_for_nix = Some(patch_binaries_for_nix); }
        if let Some(vendor) = args.vendor { self.vendor = Some(vendor); }
        if let Some(build_dir) = args.build_dir.clone() { self.build_dir = Some(build_dir); }
        if let Some(build_jobs) = args.build_jobs { self.build_jobs = Some(build_jobs); }
        if let Some(home_dir) = args.home_dir.clone() { self.home_dir = Some(home_dir); }
        if let Some(cargo_home_dir) = args.cargo_home_dir.clone() { self.cargo_home_dir = Some(cargo_home_dir); }
        if let Some(install_prefix) = args.install_prefix.clone() { self.install_prefix = Some(install_prefix); }
        if let Some(install_sysconfdir) = args.install_sysconfdir.clone() { self.install_sysconfdir = Some(install_sysconfdir); }
        if let Some(dist_sign_folder) = args.dist_sign_folder.clone() { self.dist_sign_folder = Some(dist_sign_folder); }
        if let Some(dist_upload_addr) = args.dist_upload_addr.clone() { self.dist_upload_addr = Some(dist_upload_addr); }
        if let Some(llvm_download_ci_llvm) = args.llvm_download_ci_llvm { self.llvm_download_ci_llvm = Some(llvm_download_ci_llvm); }
        if let Some(llvm_ninja) = args.llvm_ninja { self.llvm_ninja = Some(llvm_ninja); }
        if let Some(change_id) = args.change_id.clone() { self.change_id = Some(change_id); }
    }
}
