use syn::{visit::Visit, ItemConst, File, ItemFn, ItemStruct, ItemEnum, ItemStatic};
use quote::quote;

pub struct Level0DeclsVisitor {
    pub constants: Vec<ItemConst>,
    pub layer0_structs: Vec<ItemStruct>,
    pub fn_count: usize,
    pub struct_count: usize,
    pub enum_count: usize,
    pub static_count: usize,
    pub other_item_count: usize,
}

impl Level0DeclsVisitor {
    pub fn new() -> Self {
        Level0DeclsVisitor {
            constants: Vec::new(),
            layer0_structs: Vec::new(),
            fn_count: 0,
            struct_count: 0,
            enum_count: 0,
            static_count: 0,
            other_item_count: 0,
        }
    }

    pub fn extract_from_file(file: &File) -> Self {
        let mut visitor = Self::new();
        visitor.visit_file(file);
        visitor
    }
}

impl<'ast> Visit<'ast> for Level0DeclsVisitor {
    fn visit_item_const(&mut self, i: &'ast ItemConst) {
        self.constants.push(i.clone());
        syn::visit::visit_item_const(self, i);
    }

    fn visit_item_fn(&mut self, i: &'ast ItemFn) {
        self.fn_count += 1;
        syn::visit::visit_item_fn(self, i);
    }

    fn visit_item_struct(&mut self, i: &'ast ItemStruct) {
        self.struct_count += 1;
        if is_layer0_struct(i) {
            self.layer0_structs.push(i.clone());
        }
        syn::visit::visit_item_struct(self, i);
    }

    fn visit_item_enum(&mut self, i: &'ast ItemEnum) {
        self.enum_count += 1;
        syn::visit::visit_item_enum(self, i);
    }

    fn visit_item_static(&mut self, i: &'ast ItemStatic) {
        self.static_count += 1;
        syn::visit::visit_item_static(self, i);
    }

    // Catch-all for other items not explicitly handled
    fn visit_item(&mut self, i: &'ast syn::Item) {
        match i {
            syn::Item::Const(_) | syn::Item::Fn(_) | syn::Item::Struct(_) | syn::Item::Enum(_) | syn::Item::Static(_) => {},
            _ => self.other_item_count += 1,
        }
        syn::visit::visit_item(self, i);
    }
}

fn is_primitive_or_std_type(ty: &syn::Type) -> bool {
    match ty {
        syn::Type::Path(type_path) => {
            if let Some(segment) = type_path.path.segments.last() {
                let ident_str = segment.ident.to_string();
                matches!(
                    ident_str.as_str(),
                    // Primitive types
                    "bool" | "char" | "u8" | "i8" | "u16" | "i16" | "u32" | "i32" | "u64" | "i64" | "u128" | "i128" | "usize" | "isize" | "f32" | "f64" |
                    // Common std types
                    "String" | "PathBuf" | "Option" | "Vec" | "HashMap" | "Result" | "Box" | "Arc" | "Rc"
                )
            } else {
                false
            }
        },
        syn::Type::Reference(type_ref) => is_primitive_or_std_type(&type_ref.elem),
        syn::Type::Tuple(type_tuple) => type_tuple.elems.iter().all(is_primitive_or_std_type),
        syn::Type::Array(type_array) => is_primitive_or_std_type(&type_array.elem),
        _ => false,
    }
}

fn is_layer0_struct(s: &ItemStruct) -> bool {
    // Check if all fields are primitive or known std types
    s.fields.iter().all(|field| {
        is_primitive_or_std_type(&field.ty)
    })
}

pub fn generate_constants_module(constants: &[ItemConst]) -> String {
    let generated_code = constants.iter().map(|c| {
        quote! { #c }
    }).collect::<Vec<_>>();

    if generated_code.is_empty() {
        return quote! {
            // No Level 0 constant declarations found in this module.
        }.to_string();
    }

    quote! {
        // This module contains extracted Level 0 constant declarations.
        // It is automatically generated.

        #(#generated_code)*
    }.to_string()
}

pub fn generate_structs_module(structs: &[ItemStruct]) -> String {
    let generated_code = structs.iter().map(|s| {
        quote! { #s }
    }).collect::<Vec<_>>();

    if generated_code.is_empty() {
        return quote! {
            // No Level 0 struct declarations found in this module.
        }.to_string();
    }

    quote! {
        // This module contains extracted Level 0 struct declarations.
        // It is automatically generated.

        #(#generated_code)*
    }.to_string()
}
