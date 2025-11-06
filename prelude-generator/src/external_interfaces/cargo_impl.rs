use anyhow::{anyhow, Context, Result};
use std::path::PathBuf;
use std::process::Command;
use crate::external_interfaces::CargoInterface;

pub struct CargoInterfaceImpl;

impl CargoInterface for CargoInterfaceImpl {
    fn run_command(&self, args: &[&str], current_dir: Option<&PathBuf>) -> Result<String> {
        let mut command = Command::new("cargo");
        command.args(args);
        if let Some(dir) = current_dir {
            command.current_dir(dir);
        }
        let output = command.output().context("Failed to execute cargo command")?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(anyhow!("Cargo command failed: {}\nStdout: {}\nStderr: {}",
                args.join(" "),
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            ))
        }
    }

    fn metadata(&self, manifest_path: &PathBuf) -> Result<cargo_metadata::Metadata> {
        let metadata = cargo_metadata::MetadataCommand::new()
            .manifest_path(manifest_path)
            .exec()
            .context("Failed to execute cargo metadata")?;
        Ok(metadata)
    }

    fn expand_macro(&self, manifest_path: &PathBuf, lib_name: &str) -> Result<String> {
        let output = Command::new("cargo")
            .arg("rustc")
            .arg("--manifest-path")
            .arg(manifest_path)
            .arg("--lib")
            .arg("-Zunpretty=expanded")
            .arg("--")
            .arg("--crate-name")
            .arg(lib_name)
            .output()
            .context("Failed to execute cargo rustc -- -Zunpretty=expanded")?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(anyhow!("Cargo expand failed for {}: {}\nStdout: {}\nStderr: {}",
                lib_name,
                String::from_utf8_lossy(&output.stderr),
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            ))
        }
    }
}
