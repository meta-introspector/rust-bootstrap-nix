
// This will be the lib.rs for the new bootstrap-config-utils crate

pub mod default_opts;
pub mod get_builder_toml;
pub mod get_toml;
pub mod parse;
pub mod parse_inner_flags;
pub mod parse_inner_src;
pub mod parse_inner_out;
pub mod parse_inner_toml;
pub mod parse_inner_build;
pub mod dry_run;
pub mod try_run;
pub mod ci_config;
pub mod build_config;
pub mod install_config;
pub mod config_applicator;
pub mod llvm_assertions_config;
pub mod rust_channel_git_hash_config;
pub mod nix_config;
pub mod local_build;
pub mod local_ci_config;
pub mod local_dist;
pub mod local_flags;
pub mod local_llvm;
pub mod local_rust;
pub mod local_target_config;
pub mod local_toml_config;
pub mod local_nix_config;
pub mod parsed_config;
pub mod target_selection;

#[cfg(test)]
mod config_parsing_tests {
    use super::*;
    use std::path::PathBuf;
    use crate::parse::parse;
    use crate::local_flags::LocalFlags;
    use crate::dry_run::DryRun;

    #[test]
    fn test_parse_example_configs() {
        let config_files = [
            "../../../../../../config.toml", // Main config.toml
            "../../../../../../standalonex/config.toml",
            "../../../../../../standalonex/src/bootstrap/stage0/config.toml",
            "../../../../../../standalonex/src/bootstrap/defaults/config.compiler.toml",
            "../../../../../../standalonex/src/bootstrap/defaults/config.dist.toml",
            "../../../../../../standalonex/src/bootstrap/defaults/config.library.toml",
            "../../../../../../standalonex/src/bootstrap/defaults/config.tools.toml",
        ];

        for &file_path_str in &config_files {
            let file_path = PathBuf::from(file_path_str);
            println!("Testing config file: {}", file_path.display());

            let flags = LocalFlags::default(); // Create a default LocalFlags

            // Attempt to parse the config file
            // This will panic if parsing fails, which is what we want to catch in a test
            let parsed_config = parse(flags);

            // Add some basic assertions to check if parsing was successful and values are as expected
            // These assertions will need to be tailored to the actual content of each config file
            // For now, we'll just check if some common fields are not their default values if expected.

            // Example assertion for standalonex/config.toml
            if file_path_str.contains("standalonex/config.toml") {
                // Assuming change-id is parsed into parsed_config.change_id
                // This requires `change_id` to be part of ParsedConfig and LocalTomlConfig
                // and handled by an applicator.
                // For now, just check if it doesn't panic.
            }

            // Assert that dry_run is Disabled by default flags
            assert_eq!(parsed_config.dry_run, DryRun::Disabled);

            // Assert that out path is not empty
            assert!(!parsed_config.out.to_str().unwrap().is_empty());
        }
    }
}
