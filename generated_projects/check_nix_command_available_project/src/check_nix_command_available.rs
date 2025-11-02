use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

pub fn check_nix_command_available () -> Result < () > { Command :: new ("nix") . arg ("--version") . output () . with_context (| | { "Failed to execute 'nix --version'. Is Nix installed and in PATH?" }) ? . status . success () . then_some (()) . with_context (| | { "'nix' command not found or failed to execute. Please install Nix." }) }