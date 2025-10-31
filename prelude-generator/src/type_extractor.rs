use syn::{visit::Visit, Type, PathArguments, GenericArgument, ItemStruct, ItemConst, Meta};
use std::collections::HashMap;
use std::path::PathBuf;
use walkdir::WalkDir;
use anyhow::Result;

pub struct TypeInfo {
    pub count: usize,
    pub layer: Option<usize>, // None means unclassified, 0 is Level 0, 1 is Level 1, etc.
}

pub struct TypeCollector<'a> {
    pub type_map: &'a mut HashMap<String, TypeInfo>,
}

impl<'a> Visit<'a> for TypeCollector<'a> {
    fn visit_type(&mut self, i: &'a Type) {
        if let Type::Path(type_path) = i {
            for segment in type_path.path.segments.iter() {
                let type_name = segment.ident.to_string();
                let entry = self.type_map.entry(type_name.clone()).or_insert_with(|| TypeInfo { count: 0, layer: Some(0) });
                entry.count += 1;

                if is_complex_type(&type_name) {
                    entry.layer = Some(1); // Mark as Level 1 if it's a complex type
                }

                // Recursively visit generic arguments
                if let PathArguments::AngleBracketed(angle_args) = &segment.arguments {
                    for arg in angle_args.args.iter() {
                        if let GenericArgument::Type(inner_ty) = arg {
                            self.visit_type(inner_ty);
                        }
                    }
                }
            }
        }
        syn::visit::visit_type(self, i);
    }

    fn visit_item_struct(&mut self, i: &'a ItemStruct) {
        let struct_name = i.ident.to_string();
        let entry = self.type_map.entry(struct_name.clone()).or_insert_with(|| TypeInfo { count: 0, layer: Some(0) });
        entry.count += 1;

        if contains_complex_attributes(i) || contains_complex_fields(i) {
            entry.layer = Some(1); // Mark as Level 1 if it has complex attributes or fields
        }
        syn::visit::visit_item_struct(self, i);
    }

    fn visit_item_const(&mut self, i: &'a ItemConst) {
        let const_name = i.ident.to_string();
        let entry = self.type_map.entry(const_name.clone()).or_insert_with(|| TypeInfo { count: 0, layer: Some(0) });
        entry.count += 1;

        if contains_complex_attributes_for_const(i) || is_complex_type(&const_name) {
            entry.layer = Some(1); // Mark as Level 1 if it's a complex type or has complex attributes
        }
        syn::visit::visit_item_const(self, i);
    }
}

fn is_complex_type(type_name: &str) -> bool {
    type_name == "syn" || type_name == "String" || type_name == "HashMap" || type_name == "PathBuf" || type_name == "clap" || type_name == "serde"
}

fn contains_complex_attributes(structure: &ItemStruct) -> bool {
    structure.attrs.iter().any(|attr| {
        if attr.path().is_ident("derive") {
            if let Meta::List(meta_list) = &attr.meta {
                let tokens_str = meta_list.tokens.to_string();
                tokens_str.contains("Parser") || tokens_str.contains("Serialize") || tokens_str.contains("Deserialize")
            } else { false }
        } else { false }
    })
}

fn contains_complex_attributes_for_const(_constant: &ItemConst) -> bool {
    // For constants, we might not have derive attributes in the same way as structs.
    // This can be expanded if needed.
    false
}

fn contains_complex_fields(structure: &ItemStruct) -> bool {
    for field in structure.fields.iter() {
        if let Type::Path(type_path) = &field.ty {
            for segment in type_path.path.segments.iter() {
                let ident_str = segment.ident.to_string();
                if is_complex_type(&ident_str) {
                    return true;
                }
                if let PathArguments::AngleBracketed(angle_args) = &segment.arguments {
                    for arg in angle_args.args.iter() {
                        if let GenericArgument::Type(inner_ty) = arg {
                            if contains_complex_type_in_type(inner_ty) {
                                return true;
                            }
                        }
                    }
                }
            }
        }
    }
    false
}

fn contains_complex_type_in_type(ty: &Type) -> bool {
    match ty {
        Type::Path(type_path) => {
            for segment in type_path.path.segments.iter() {
                let ident_str = segment.ident.to_string();
                if is_complex_type(&ident_str) {
                    return true;
                }
                if let PathArguments::AngleBracketed(angle_args) = &segment.arguments {
                    for arg in angle_args.args.iter() {
                        if let GenericArgument::Type(inner_ty) = arg {
                            if contains_complex_type_in_type(inner_ty) {
                                return true;
                            }
                        }
                    }
                }
            }
            false
        },
        _ => false,
    }
}

pub async fn extract_bag_of_types(project_root: &PathBuf) -> Result<HashMap<String, TypeInfo>> {
    let mut type_map: HashMap<String, TypeInfo> = HashMap::new();

    for entry in WalkDir::new(project_root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file() && e.path().extension().map_or(false, |ext| ext == "rs"))
    {
        let path = entry.path();
        let content = std::fs::read_to_string(&path)?;
        let file = syn::parse_file(&content)?;

        let mut collector = TypeCollector {
            type_map: &mut type_map,
        };
        collector.visit_file(&file);
    }

    Ok(type_map)
}