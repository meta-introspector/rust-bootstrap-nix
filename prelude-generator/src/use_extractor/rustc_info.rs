use anyhow::Result;
use std::process::Command;

// Struct to hold rustc version and host triple
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RustcInfo {
    pub version: String,
    pub host: String,
}

pub fn get_rustc_info() -> Result<RustcInfo> {
    let output = Command::new("rustc")
        .arg("--version")
        .arg("--verbose")
        .output()?;

    if !output.status.success() {
        anyhow::bail!(
            "rustc --version --verbose failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let version_line = stdout.lines().find(|line| line.starts_with("rustc "));
    let host_line = stdout.lines().find(|line| line.starts_with("host: "));

    let version = version_line
        .and_then(|line| line.split_whitespace().nth(1))
        .unwrap_or("unknown")
        .to_string();
    let host = host_line
        .and_then(|line| line.split_whitespace().nth(1))
        .unwrap_or("unknown")
        .to_string();

    Ok(RustcInfo { version, host })
}
