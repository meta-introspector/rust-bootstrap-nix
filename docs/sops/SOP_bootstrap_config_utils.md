# SOP: `bootstrap-config-utils` Crate

## 1. Purpose

The `bootstrap-config-utils` crate is a foundational component within the Rust bootstrap process. Its primary purpose is to provide a self-contained, "layer 1" utility for parsing, validating, and preparing configuration inputs for the larger Rust build system. It aims to be free of direct dependencies on the main `bootstrap` crate types, ensuring a clean separation of concerns and improved modularity.

This crate is responsible for:
- Reading configuration from various sources (e.g., `config.toml`, environment variables, command-line flags).
- Deserializing TOML configuration into structured Rust types.
- Applying configuration flags and settings to a unified `ParsedConfig` struct.
- Providing a validated and consolidated configuration object that can be used by subsequent build stages.

## 2. Key Components

### `ParsedConfig` Struct
The central data structure of this crate, `ParsedConfig`, holds the consolidated and validated configuration for the Rust build. It is designed to be a comprehensive representation of all configurable options, independent of the `bootstrap` crate's internal `Config` type.

### `LocalFlags` Struct
Represents command-line flags passed to the bootstrap process. This struct is used to initially capture user-provided options before they are applied to the `ParsedConfig`.

### `LocalTomlConfig` Struct
Represents the structure of the `config.toml` file, allowing for deserialization of user-defined build settings.

### `ConfigApplicator` Trait
A trait that defines a standard interface for applying specific configuration sections (e.g., CI, build, install) from `LocalTomlConfig` to the `ParsedConfig`. This promotes modularity and extensibility in how configuration is processed.

### Modules for Configuration Parsing
The crate includes several modules (e.g., `parse_inner_src`, `parse_inner_out`, `parse_inner_toml`, `parse_inner_build`, `parse_inner_flags`) that handle the parsing and application of different parts of the configuration. The main `parse.rs` module orchestrates these individual parsing steps.

### `DryRun` Enum
An enum used to indicate whether the build process should perform a dry run, allowing for checks without actual execution.

### `TargetSelection` Tuple Struct
A simple tuple struct used to encapsulate target triple strings, providing a type-safe way to handle build and host targets.

## 3. Usage

The `bootstrap-config-utils` crate is typically used early in the Rust bootstrap process. Its main entry point for configuration processing is the `parse` function, which takes `LocalFlags` as input and returns a fully populated `ParsedConfig` object.

```rust
// Example usage (simplified)
use bootstrap_config_utils::parse;
use bootstrap_config_utils::local_flags::LocalFlags;

fn main() {
    // Simulate command-line flags
    let flags = LocalFlags {
        // ... populate with actual flags or defaults
        ..Default::default()
    };

    // Parse and get the consolidated configuration
    let config = parse(flags);

    // Now 'config' contains the validated build configuration
    // ... proceed with build logic using 'config'
}
```

## 4. Development and Maintenance

- **Modularity:** Changes should adhere to the principle of keeping `bootstrap-config-utils` as a "layer 1" crate, minimizing dependencies on higher-level `bootstrap` types.
- **Testing:** Ensure that any changes to parsing logic or configuration application are thoroughly tested to prevent regressions.
- **Error Handling:** Robust error handling is crucial for providing clear feedback to users about invalid configurations.
- **Documentation:** Keep this documentation up-to-date with any significant changes to the crate's structure or functionality.
