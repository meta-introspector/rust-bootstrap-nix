use std::collections::{HashSet, HashMap};
use std::path::PathBuf;
use syn::{self, visit::{self, Visit}, ItemConst, ItemStruct, ItemEnum, ItemFn, ItemStatic};
use quote::{quote, ToTokens};

use crate::types::{Declaration, DeclarationItem}; // Assuming types are re-exported from lib.rs

struct DependencyVisitor {
    required_imports: HashSet<String>,
    _known_identifiers: HashSet<String>,
}

impl<'ast> Visit<'ast> for DependencyVisitor {
    fn visit_path(&mut self, path: &'ast syn::Path) {
        let path_str = quote! { #path }.to_string();
        // We don't have direct access to verbosity here, so we'll rely on the caller (DeclsVisitor) to print if needed.
        self.required_imports.insert(path_str);
        visit::visit_path(self, path);
    }

    fn visit_item(&mut self, i: &'ast syn::Item) {
        visit::visit_item(self, i);
    }

    fn visit_trait_item(&mut self, i: &'ast syn::TraitItem) {
        visit::visit_trait_item(self, i);
    }

    fn visit_type(&mut self, i: &'ast syn::Type) {
        let type_str = quote! { #i }.to_string();
        self.required_imports.insert(type_str);
        visit::visit_type(self, i);
    }

    fn visit_expr(&mut self, i: &'ast syn::Expr) {
        let expr_str = quote! { #i }.to_string();
        self.required_imports.insert(expr_str);
        visit::visit_expr(self, i);
    }
}

pub struct DeclsVisitor {
    pub declarations: HashMap<String, Declaration>,
    pub fn_count: usize,
    pub struct_count: usize,
    pub enum_count: usize,
    pub const_count: usize,
    pub static_count: usize,
    pub macro_count: usize,
    pub mod_count: usize,
    pub trait_count: usize,
    pub trait_alias_count: usize,
    pub type_count: usize,
    pub union_count: usize,
    pub other_item_count: usize,
    pub source_file: PathBuf,
    pub crate_name: String,
    pub verbosity: u8,
    pub file_extern_crates: HashSet<String>,
}

impl DeclsVisitor {
    pub fn new(source_file: PathBuf, crate_name: String, verbosity: u8, file_extern_crates: HashSet<String>) -> Self {
        DeclsVisitor {
            declarations: HashMap::new(),
            fn_count: 0,
            struct_count: 0,
            enum_count: 0,
            const_count: 0,
            static_count: 0,
            macro_count: 0,
            mod_count: 0,
            trait_count: 0,
            trait_alias_count: 0,
            type_count: 0,
            union_count: 0,
            other_item_count: 0,
            source_file,
            crate_name,
            verbosity,
            file_extern_crates,
        }
    }

    fn is_proc_macro_item(attrs: &[syn::Attribute]) -> bool {
        attrs.iter().any(|attr| {
            attr.path().is_ident("proc_macro") ||
            attr.path().is_ident("proc_macro_derive") ||
            attr.path().is_ident("proc_macro_attribute")
        })
    }

    fn extract_identifiers_from_type(&self, ty: &syn::Type) -> HashSet<String> {
        let mut identifiers = HashSet::new();
        match ty {
            syn::Type::Path(type_path) => {
                for segment in type_path.path.segments.iter() {
                    identifiers.insert(segment.ident.to_string());
                }
            },
            _ => {} // Handle other types if necessary
        }
        identifiers
    }

    fn extract_identifiers_from_expr(&self, expr: &syn::Expr) -> HashSet<String> {
        let mut identifiers = HashSet::new();
        match expr {
            syn::Expr::Path(expr_path) => {
                for segment in expr_path.path.segments.iter() {
                    identifiers.insert(segment.ident.to_string());
                }
            },
            syn::Expr::Call(expr_call) => {
                identifiers.extend(self.extract_identifiers_from_expr(&expr_call.func));
            },
            syn::Expr::MethodCall(expr_method_call) => {
                identifiers.insert(expr_method_call.method.to_string());
                identifiers.extend(self.extract_identifiers_from_expr(&expr_method_call.receiver));
            },
            _ => {} // Handle other expression types if necessary
        }
        identifiers
    }
}

impl<'ast> Visit<'ast> for DeclsVisitor {
    fn visit_item_const(&mut self, i: &'ast ItemConst) {
        let is_public = matches!(i.vis, syn::Visibility::Public(_));

        let mut dependency_visitor = DependencyVisitor {
            required_imports: HashSet::new(),
            _known_identifiers: HashSet::new(),
        };
        dependency_visitor.visit_item_const(i);

        let decl = Declaration::new(
            DeclarationItem::Const(i.to_token_stream().to_string()),
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
            self.source_file.clone(),
            self.crate_name.clone(),
            false,
            dependency_visitor.required_imports,
            self.file_extern_crates.clone(),
            is_public,
        );
        let identifier = decl.get_identifier();
        self.declarations.insert(identifier, decl);
        self.const_count += 1;
    }

    fn visit_item_struct(&mut self, i: &'ast ItemStruct) {
        let mut referenced_types = HashSet::new();
        for field in i.fields.iter() {
            referenced_types.extend(self.extract_identifiers_from_type(&field.ty));
        }

        let is_proc_macro = Self::is_proc_macro_item(&i.attrs);
        let is_public = matches!(i.vis, syn::Visibility::Public(_));

        let mut dependency_visitor = DependencyVisitor {
            required_imports: HashSet::new(),
            _known_identifiers: HashSet::new(),
        };
        dependency_visitor.visit_item_struct(i);

        let decl = Declaration::new(
            DeclarationItem::Struct(i.to_token_stream().to_string()),
            referenced_types,
            HashSet::new(),
            HashSet::new(),
            self.source_file.clone(),
            self.crate_name.clone(),
            is_proc_macro,
            dependency_visitor.required_imports,
            self.file_extern_crates.clone(),
            is_public,
        );
        let identifier = decl.get_identifier();
        self.declarations.insert(identifier, decl);
        self.struct_count += 1;
        syn::visit::visit_item_struct(self, i);
    }

    fn visit_item_enum(&mut self, i: &'ast ItemEnum) {
        let is_proc_macro = Self::is_proc_macro_item(&i.attrs);
        let is_public = matches!(i.vis, syn::Visibility::Public(_));

        let mut dependency_visitor = DependencyVisitor {
            required_imports: HashSet::new(),
            _known_identifiers: HashSet::new(),
        };
        dependency_visitor.visit_item_enum(i);

        let decl = Declaration::new(
            DeclarationItem::Enum(i.to_token_stream().to_string()),
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
            self.source_file.clone(),
            self.crate_name.clone(),
            is_proc_macro,
            dependency_visitor.required_imports,
            self.file_extern_crates.clone(),
            is_public,
        );
        let identifier = decl.get_identifier();
        self.declarations.insert(identifier, decl);
        self.enum_count += 1;
    }

    fn visit_item_fn(&mut self, i: &'ast ItemFn) {
        let mut referenced_types = HashSet::new();
        let mut referenced_functions = HashSet::new();

        for input in i.sig.inputs.iter() {
            if let syn::FnArg::Typed(pat_type) = input {
                referenced_types.extend(self.extract_identifiers_from_type(&pat_type.ty));
            }
        }

        if let syn::ReturnType::Type(_, ty) = &i.sig.output {
            referenced_types.extend(self.extract_identifiers_from_type(ty));
        }

        struct FunctionBodyVisitor<'b> {
            parent: &'b mut DeclsVisitor,
            referenced_types: &'b mut HashSet<String>,
            referenced_functions: &'b mut HashSet<String>,
        }

        impl<'ast, 'b> Visit<'ast> for FunctionBodyVisitor<'b> {
            fn visit_expr(&mut self, i: &'ast syn::Expr) {
                self.referenced_types.extend(self.parent.extract_identifiers_from_expr(i));
                self.referenced_functions.extend(self.parent.extract_identifiers_from_expr(i));
                syn::visit::visit_expr(self, i);
            }

            fn visit_type(&mut self, i: &'ast syn::Type) {
                self.referenced_types.extend(self.parent.extract_identifiers_from_type(i));
                syn::visit::visit_type(self, i);
            }
        }

        let mut body_visitor = FunctionBodyVisitor {
            parent: self,
            referenced_types: &mut referenced_types,
            referenced_functions: &mut referenced_functions,
        };
        body_visitor.visit_block(&i.block);

        let is_proc_macro = Self::is_proc_macro_item(&i.attrs);
        let is_public = matches!(i.vis, syn::Visibility::Public(_));

        let mut dependency_visitor = DependencyVisitor {
            required_imports: HashSet::new(),
            _known_identifiers: HashSet::new(),
        };
        dependency_visitor.visit_item_fn(i);

        let decl = Declaration::new(
            DeclarationItem::Fn(i.to_token_stream().to_string()),
            referenced_types,
            referenced_functions,
            HashSet::new(),
            self.source_file.clone(),
            self.crate_name.clone(),
            is_proc_macro,
            dependency_visitor.required_imports,
            self.file_extern_crates.clone(),
            is_public,
        );
        let identifier = decl.get_identifier();
        self.declarations.insert(identifier, decl);
        self.fn_count += 1;
    }

    fn visit_item_static(&mut self, i: &'ast ItemStatic) {
        let is_public = matches!(i.vis, syn::Visibility::Public(_));

        let mut dependency_visitor = DependencyVisitor {
            required_imports: HashSet::new(),
            _known_identifiers: HashSet::new(),
        };
        dependency_visitor.visit_item_static(i);

        let decl = Declaration::new(
            DeclarationItem::Static(i.to_token_stream().to_string()),
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
            self.source_file.clone(),
            self.crate_name.clone(),
            false,
            dependency_visitor.required_imports,
            self.file_extern_crates.clone(),
            is_public,
        );
        let identifier = decl.get_identifier();
        self.declarations.insert(identifier, decl);
        self.static_count += 1;
    }

    fn visit_item_macro(&mut self, i: &'ast syn::ItemMacro) {
        let is_proc_macro = false;
        let is_public = false; // Macros do not have a direct visibility attribute

        let mut dependency_visitor = DependencyVisitor {
            required_imports: HashSet::new(),
            _known_identifiers: HashSet::new(),
        };
        dependency_visitor.visit_item_macro(i);

        let decl = Declaration::new(
            DeclarationItem::Macro(i.to_token_stream().to_string()),
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
            self.source_file.clone(),
            self.crate_name.clone(),
            is_proc_macro,
            dependency_visitor.required_imports,
            self.file_extern_crates.clone(),
            is_public,
        );
        let identifier = decl.get_identifier();
        self.declarations.insert(identifier, decl);
        self.macro_count += 1;
    }

    fn visit_item_mod(&mut self, i: &'ast syn::ItemMod) {
        let is_public = matches!(i.vis, syn::Visibility::Public(_));

        let mut dependency_visitor = DependencyVisitor {
            required_imports: HashSet::new(),
            _known_identifiers: HashSet::new(),
        };

        // Recursively visit items within the module to collect dependencies
        if let Some((_, items)) = &i.content {
            for item in items {
                dependency_visitor.visit_item(item);
            }
        }

        // Create a Declaration for the module itself
        let decl = Declaration::new(
            DeclarationItem::Mod(i.to_token_stream().to_string()),
            HashSet::new(), // referenced_types (not directly applicable to module itself)
            HashSet::new(), // referenced_functions (not directly applicable to module itself)
            HashSet::new(),
            self.source_file.clone(),
            self.crate_name.clone(),
            false,
            dependency_visitor.required_imports,
            self.file_extern_crates.clone(),
            is_public,
        );
        let identifier = decl.get_identifier();
        if !identifier.is_empty() && self.verbosity >= 2 {
            println!("  [split-expanded-lib] DeclsVisitor: Visiting module '{}'. Required imports for module: {:?}", identifier, decl.required_imports);
        }
        self.declarations.insert(identifier, decl);
        self.mod_count += 1;

        // Do not call syn::visit::visit_item_mod(self, i) here,
        // as we are manually recursing into the module's content.
    }

    fn visit_item_trait(&mut self, i: &'ast syn::ItemTrait) {
        let is_public = matches!(i.vis, syn::Visibility::Public(_));

        let mut dependency_visitor = DependencyVisitor {
            required_imports: HashSet::new(),
            _known_identifiers: HashSet::new(),
        };

        // Visit items within the trait to collect dependencies
        for item in &i.items {
            dependency_visitor.visit_trait_item(item);
        }

        let decl = Declaration::new(
            DeclarationItem::Trait(i.to_token_stream().to_string()),
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
            self.source_file.clone(),
            self.crate_name.clone(),
            false,
            dependency_visitor.required_imports,
            self.file_extern_crates.clone(),
            is_public,
        );
        let identifier = decl.get_identifier();
        self.declarations.insert(identifier, decl);
        self.trait_count += 1;
        // Do not call syn::visit::visit_item_trait(self, i) here,
        // as we are manually recursing into the trait's content.
    }

    fn visit_item_trait_alias(&mut self, i: &'ast syn::ItemTraitAlias) {
        let is_public = matches!(i.vis, syn::Visibility::Public(_));

        let decl = Declaration::new(
            DeclarationItem::TraitAlias(i.to_token_stream().to_string()),
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
            self.source_file.clone(),
            self.crate_name.clone(),
            false,
            HashSet::new(),
            self.file_extern_crates.clone(),
            is_public,
        );
        let identifier = decl.get_identifier();
        self.declarations.insert(identifier, decl);
        self.trait_alias_count += 1;
    }

    fn visit_item_type(&mut self, i: &'ast syn::ItemType) {
        let is_public = matches!(i.vis, syn::Visibility::Public(_));

        let decl = Declaration::new(
            DeclarationItem::Type(i.to_token_stream().to_string()),
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
            self.source_file.clone(),
            self.crate_name.clone(),
            false,
            HashSet::new(),
            self.file_extern_crates.clone(),
            is_public,
        );
        let identifier = decl.get_identifier();
        self.declarations.insert(identifier, decl);
        self.type_count += 1;
    }

    fn visit_item_union(&mut self, i: &'ast syn::ItemUnion) {
        let is_public = matches!(i.vis, syn::Visibility::Public(_));

        let decl = Declaration::new(
            DeclarationItem::Union(i.to_token_stream().to_string()),
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
            self.source_file.clone(),
            self.crate_name.clone(),
            false,
            HashSet::new(),
            self.file_extern_crates.clone(),
            is_public,
        );
        let identifier = decl.get_identifier();
        self.declarations.insert(identifier, decl);
        self.union_count += 1;
    }

    fn visit_item(&mut self, i: &'ast syn::Item) {
        let is_public = match i {
            syn::Item::Const(item_const) => matches!(item_const.vis, syn::Visibility::Public(_)),
            syn::Item::Enum(item_enum) => matches!(item_enum.vis, syn::Visibility::Public(_)),
            syn::Item::Fn(item_fn) => matches!(item_fn.vis, syn::Visibility::Public(_)),
            syn::Item::Mod(item_mod) => matches!(item_mod.vis, syn::Visibility::Public(_)),
            syn::Item::Static(item_static) => matches!(item_static.vis, syn::Visibility::Public(_)),
            syn::Item::Struct(item_struct) => matches!(item_struct.vis, syn::Visibility::Public(_)),
            syn::Item::Trait(item_trait) => matches!(item_trait.vis, syn::Visibility::Public(_)),
            syn::Item::TraitAlias(item_trait_alias) => matches!(item_trait_alias.vis, syn::Visibility::Public(_)),
            syn::Item::Type(item_type) => matches!(item_type.vis, syn::Visibility::Public(_)),
            syn::Item::Union(item_union) => matches!(item_union.vis, syn::Visibility::Public(_)),
            _ => false, // For other item types, assume not public for now
        };

        let decl = Declaration::new(
            DeclarationItem::Other(i.to_token_stream().to_string()),
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
            self.source_file.clone(),
            self.crate_name.clone(),
            false,
            HashSet::new(),
            self.file_extern_crates.clone(),
            is_public,
        );
        let identifier = decl.get_identifier();
        self.declarations.insert(identifier, decl);
        self.other_item_count += 1;
    }
}