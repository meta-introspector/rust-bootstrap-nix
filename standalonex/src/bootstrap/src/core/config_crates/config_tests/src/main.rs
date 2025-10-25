use crate::prelude::*;


use config_macros::define_config;
use config_core::Merge;

define_config! {
    struct MyConfig {
        field1: Option<String> = "field1",
        field2: Option<u32> = "field2",
    }
}

define_config! {
    #[derive(Clone)]
    struct ComplexConfig {
        name: Option<String> = "name_key",
        version: Option<f64> = "version_key",
        enabled: Option<bool> = "enabled_key",
        count: Option<usize> = "count_key",
        path: Option<std::path::PathBuf> = "path_key",
    }
}

fn main() {
    // Test MyConfig
    let mut config1 = MyConfig {
        field1: Some("value1".to_string()),
        field2: None,
    };

    let config2 = MyConfig {
        field1: None,
        field2: Some(123),
    };

    config1.merge(config2, config_core::ReplaceOpt::IgnoreDuplicate);

    assert_eq!(config1.field1, Some("value1".to_string()));
    assert_eq!(config1.field2, Some(123));

    println!("MyConfig tests passed!");

    // Test ComplexConfig
    let complex_config_a = ComplexConfig {
        name: Some("AppA".to_string()),
        version: Some(1.0),
        enabled: Some(true),
        count: None,
        path: Some("/tmp/a".into()),
    };

    let complex_config_b = ComplexConfig {
        name: Some("AppB".to_string()),
        version: Some(2.0),
        enabled: None,
        count: Some(100),
        path: None,
    };

    // Test ReplaceOpt::Override
    let mut merged_override = complex_config_a.clone();
    merged_override.merge(complex_config_b.clone(), config_core::ReplaceOpt::Override);
    assert_eq!(merged_override.name, Some("AppB".to_string()));
    assert_eq!(merged_override.version, Some(2.0));
    assert_eq!(merged_override.enabled, Some(true));
    assert_eq!(merged_override.count, Some(100));
    assert_eq!(merged_override.path, Some("/tmp/a".into()));

    println!("ComplexConfig ReplaceOpt::Override tests passed!");

    // Test ReplaceOpt::IgnoreDuplicate
    let mut merged_ignore = complex_config_a.clone();
    merged_ignore.merge(complex_config_b.clone(), config_core::ReplaceOpt::IgnoreDuplicate);
    assert_eq!(merged_ignore.name, Some("AppA".to_string()));
    assert_eq!(merged_ignore.version, Some(1.0));
    assert_eq!(merged_ignore.enabled, Some(true));
    assert_eq!(merged_ignore.count, Some(100));
    assert_eq!(merged_ignore.path, Some("/tmp/a".into()));

    println!("ComplexConfig ReplaceOpt::IgnoreDuplicate tests passed!");

    // Test ReplaceOpt::ErrorOnDuplicate (should panic if a duplicate is found)
    let complex_config_c = complex_config_a.clone(); // Use a clone to move into the closure
    let res = std::panic::catch_unwind(move || {
        let mut merged_error = complex_config_c;
        merged_error.merge(complex_config_b.clone(), config_core::ReplaceOpt::ErrorOnDuplicate);
    });
    assert!(res.is_err());

    println!("ComplexConfig ReplaceOpt::ErrorOnDuplicate tests passed (panic caught)!
");

    println!("All tests passed!");
}