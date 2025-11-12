use std::collections::HashMap;
use crate::declaration::ResolvedDependency;

pub fn get_rust_language_symbols() -> HashMap<String, ResolvedDependency> {
    let mut symbols = HashMap::new();

    // Primitive types
    add_primitive_type(&mut symbols, "bool");
    add_primitive_type(&mut symbols, "char");
    add_primitive_type(&mut symbols, "f32");
    add_primitive_type(&mut symbols, "f64");
    add_primitive_type(&mut symbols, "i8");
    add_primitive_type(&mut symbols, "i16");
    add_primitive_type(&mut symbols, "i32");
    add_primitive_type(&mut symbols, "i64");
    add_primitive_type(&mut symbols, "i128");
    add_primitive_type(&mut symbols, "isize");
    add_primitive_type(&mut symbols, "u8");
    add_primitive_type(&mut symbols, "u16");
    add_primitive_type(&mut symbols, "u32");
    add_primitive_type(&mut symbols, "u64");
    add_primitive_type(&mut symbols, "u128");
    add_primitive_type(&mut symbols, "usize");
    add_primitive_type(&mut symbols, "str");

    // Common standard library types/modules
    add_std_lib_symbol(&mut symbols, "std", "module", None);
    add_std_lib_symbol(&mut symbols, "String", "type", Some("std::string"));
    add_std_lib_symbol(&mut symbols, "Vec", "type", Some("std::vec"));
    add_std_lib_symbol(&mut symbols, "HashMap", "type", Some("std::collections"));
    add_std_lib_symbol(&mut symbols, "HashSet", "type", Some("std::collections"));
    add_std_lib_symbol(&mut symbols, "Option", "enum", Some("std::option"));
    add_std_lib_symbol(&mut symbols, "Result", "enum", Some("std::result"));
    add_std_lib_symbol(&mut symbols, "PathBuf", "type", Some("std::path"));
    add_std_lib_symbol(&mut symbols, "Path", "type", Some("std::path"));
    add_std_lib_symbol(&mut symbols, "fs", "module", Some("std"));
    add_std_lib_symbol(&mut symbols, "io", "module", Some("std"));
    add_std_lib_symbol(&mut symbols, "fmt", "module", Some("std"));
    add_std_lib_symbol(&mut symbols, "collections", "module", Some("std"));

    // Common macros (represented as functions for simplicity in SymbolMap)
    add_std_lib_symbol(&mut symbols, "println!", "macro", Some("std::macro"));
    add_std_lib_symbol(&mut symbols, "eprintln!", "macro", Some("std::macro"));
    add_std_lib_symbol(&mut symbols, "vec!", "macro", Some("std::macro"));
    add_std_lib_symbol(&mut symbols, "assert!", "macro", Some("std::macro"));
    add_std_lib_symbol(&mut symbols, "assert_eq!", "macro", Some("std::macro"));

    symbols
}

fn add_primitive_type(symbols: &mut HashMap<String, ResolvedDependency>, id: &str) {
    symbols.insert(id.to_string(), ResolvedDependency {
        id: id.to_string(),
        dependency_type: "primitive_type".to_string(),
        crate_name: "std".to_string(),
        module_path: "std".to_string(),
        usage_count: 0,
    });
}

fn add_std_lib_symbol(symbols: &mut HashMap<String, ResolvedDependency>, id: &str, dep_type: &str, module_path: Option<&str>) {
    symbols.insert(id.to_string(), ResolvedDependency {
        id: id.to_string(),
        dependency_type: dep_type.to_string(),
        crate_name: "std".to_string(),
        module_path: module_path.unwrap_or("std").to_string(),
        usage_count: 0,
    });
}
