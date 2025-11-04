use anyhow::Context;
use std::path::{PathBuf, Path};
use syn::{self, visit::{self, Visit}, ItemConst, ItemStruct, ItemEnum, ItemFn, ItemStatic};

use quote::quote;

use std::collections::{HashSet, HashMap};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Default)]
pub struct FileMetadata {
    pub global_uses: HashSet<String>,
    pub feature_attributes: HashSet<String>,
    pub extern_crates: HashSet<String>,
}

#[derive(Debug)]
pub enum DeclarationItem {
    Const(ItemConst),
    Struct(ItemStruct),
    Enum(ItemEnum),
    Fn(ItemFn),
    Static(ItemStatic),
    Macro(syn::ItemMacro),
    Mod(syn::ItemMod),
    Trait(syn::ItemTrait),
    TraitAlias(syn::ItemTraitAlias),
    Type(syn::ItemType),
    Union(syn::ItemUnion),
    Other(syn::Item),
}

#[derive(Debug)]
pub struct Declaration {
    pub item: DeclarationItem,
    pub referenced_types: HashSet<String>,
    pub referenced_functions: HashSet<String>,
    pub external_identifiers: HashSet<String>,
    pub source_file: PathBuf,
    pub crate_name: String,
    pub resolved_dependencies: HashSet<String>,
    pub is_proc_macro: bool,
    pub required_imports: HashSet<String>,
    pub direct_dependencies: HashSet<String>,
}

impl Declaration {
    pub fn new(
        item: DeclarationItem,
        referenced_types: HashSet<String>,
        referenced_functions: HashSet<String>,
        external_identifiers: HashSet<String>,
        source_file: PathBuf,
        crate_name: String,
        is_proc_macro: bool,
        required_imports: HashSet<String>,
    ) -> Self {
        Declaration {
            item,
            referenced_types,
            referenced_functions,
            external_identifiers,
            source_file,
            crate_name,
            resolved_dependencies: HashSet::new(),
            is_proc_macro,
            required_imports,
            direct_dependencies: HashSet::new(),
        }
    }

    pub fn get_identifier(&self) -> String {
        match &self.item {
            DeclarationItem::Const(item_const) => item_const.ident.to_string(),
            DeclarationItem::Struct(item_struct) => item_struct.ident.to_string(),
            DeclarationItem::Enum(item_enum) => item_enum.ident.to_string(),
            DeclarationItem::Fn(item_fn) => item_fn.sig.ident.to_string(),
            DeclarationItem::Static(item_static) => item_static.ident.to_string(),
            DeclarationItem::Macro(item_macro) => item_macro.ident.as_ref().map_or_else(|| "unknown_macro".to_string(), |ident| ident.to_string()),
            DeclarationItem::Mod(item_mod) => item_mod.ident.to_string(),
            DeclarationItem::Trait(item_trait) => item_trait.ident.to_string(),
            DeclarationItem::TraitAlias(item_trait_alias) => item_trait_alias.ident.to_string(),
            DeclarationItem::Type(item_type) => item_type.ident.to_string(),
            DeclarationItem::Union(item_union) => item_union.ident.to_string(),
            DeclarationItem::Other(_) => {
                "unknown_declaration".to_string()
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorSample {
    pub file_path: PathBuf,
    pub rustc_version: String,
    pub rustc_host: String,
    pub error_message: String,
    pub error_type: String,
    pub code_snippet: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub context: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RustcInfo {
    pub version: String,
    pub host: String,
}

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
}

impl DeclsVisitor {
    pub fn new(source_file: PathBuf, crate_name: String, verbosity: u8) -> Self {
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
        let mut dependency_visitor = DependencyVisitor {
            required_imports: HashSet::new(),
            _known_identifiers: HashSet::new(),
        };
        dependency_visitor.visit_item_const(i);

        let decl = Declaration::new(
            DeclarationItem::Const(i.clone()),
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
            self.source_file.clone(),
            self.crate_name.clone(),
            false,
            dependency_visitor.required_imports,
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

        let mut dependency_visitor = DependencyVisitor {
            required_imports: HashSet::new(),
            _known_identifiers: HashSet::new(),
        };
        dependency_visitor.visit_item_struct(i);

        let decl = Declaration::new(
            DeclarationItem::Struct(i.clone()),
            referenced_types,
            HashSet::new(),
            HashSet::new(),
            self.source_file.clone(),
            self.crate_name.clone(),
            is_proc_macro,
            dependency_visitor.required_imports,
        );
        let identifier = decl.get_identifier();
        self.declarations.insert(identifier, decl);
        self.struct_count += 1;
        syn::visit::visit_item_struct(self, i);
    }

    fn visit_item_enum(&mut self, i: &'ast ItemEnum) {
        let is_proc_macro = Self::is_proc_macro_item(&i.attrs);

        let mut dependency_visitor = DependencyVisitor {
            required_imports: HashSet::new(),
            _known_identifiers: HashSet::new(),
        };
        dependency_visitor.visit_item_enum(i);

        let decl = Declaration::new(
            DeclarationItem::Enum(i.clone()),
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
            self.source_file.clone(),
            self.crate_name.clone(),
            is_proc_macro,
            dependency_visitor.required_imports,
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

        let mut dependency_visitor = DependencyVisitor {
            required_imports: HashSet::new(),
            _known_identifiers: HashSet::new(),
        };
        dependency_visitor.visit_item_fn(i);

        let decl = Declaration::new(
            DeclarationItem::Fn(i.clone()),
            referenced_types,
            referenced_functions,
            HashSet::new(),
            self.source_file.clone(),
            self.crate_name.clone(),
            is_proc_macro,
            dependency_visitor.required_imports,
        );
        let identifier = decl.get_identifier();
        self.declarations.insert(identifier, decl);
        self.fn_count += 1;
    }

    fn visit_item_static(&mut self, i: &'ast ItemStatic) {
        let mut dependency_visitor = DependencyVisitor {
            required_imports: HashSet::new(),
            _known_identifiers: HashSet::new(),
        };
        dependency_visitor.visit_item_static(i);

        let decl = Declaration::new(
            DeclarationItem::Static(i.clone()),
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
            self.source_file.clone(),
            self.crate_name.clone(),
            false,
            dependency_visitor.required_imports,
        );
        let identifier = decl.get_identifier();
        self.declarations.insert(identifier, decl);
        self.static_count += 1;
    }

    fn visit_item_macro(&mut self, i: &'ast syn::ItemMacro) {
        let is_proc_macro = false;

        let mut dependency_visitor = DependencyVisitor {
            required_imports: HashSet::new(),
            _known_identifiers: HashSet::new(),
        };
        dependency_visitor.visit_item_macro(i);

        let decl = Declaration::new(
            DeclarationItem::Macro(i.clone()),
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
            self.source_file.clone(),
            self.crate_name.clone(),
            is_proc_macro,
            dependency_visitor.required_imports,
        );
        let identifier = decl.get_identifier();
        self.declarations.insert(identifier, decl);
        self.macro_count += 1;
    }

    fn visit_item_mod(&mut self, i: &'ast syn::ItemMod) {
        let mut dependency_visitor = DependencyVisitor {
            required_imports: HashSet::new(),
            _known_identifiers: HashSet::new(), // This will be populated later if needed
        };

        // Recursively visit items within the module to collect dependencies
        if let Some((_, items)) = &i.content {
            for item in items {
                dependency_visitor.visit_item(item);
            }
        }

        // Create a Declaration for the module itself
        let decl = Declaration::new(
            DeclarationItem::Mod(i.clone()),
            HashSet::new(), // referenced_types (not directly applicable to module itself)
            HashSet::new(), // referenced_functions (not directly applicable to module itself)
            HashSet::new(),
            self.source_file.clone(),
            self.crate_name.clone(),
            false,
            dependency_visitor.required_imports,
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
        let mut dependency_visitor = DependencyVisitor {
            required_imports: HashSet::new(),
            _known_identifiers: HashSet::new(),
        };

        // Visit items within the trait to collect dependencies
        for item in &i.items {
            dependency_visitor.visit_trait_item(item);
        }

        let decl = Declaration::new(
            DeclarationItem::Trait(i.clone()),
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
            self.source_file.clone(),
            self.crate_name.clone(),
            false,
            dependency_visitor.required_imports,
        );
        let identifier = decl.get_identifier();
        self.declarations.insert(identifier, decl);
        self.trait_count += 1;
        // Do not call syn::visit::visit_item_trait(self, i) here,
        // as we are manually recursing into the trait's content.
    }

    fn visit_item_trait_alias(&mut self, i: &'ast syn::ItemTraitAlias) {
        let decl = Declaration::new(
            DeclarationItem::TraitAlias(i.clone()),
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
            self.source_file.clone(),
            self.crate_name.clone(),
            false,
            HashSet::new(),
        );
        let identifier = decl.get_identifier();
        self.declarations.insert(identifier, decl);
        self.trait_alias_count += 1;
    }

    fn visit_item_type(&mut self, i: &'ast syn::ItemType) {
        let decl = Declaration::new(
            DeclarationItem::Type(i.clone()),
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
            self.source_file.clone(),
            self.crate_name.clone(),
            false,
            HashSet::new(),
        );
        let identifier = decl.get_identifier();
        self.declarations.insert(identifier, decl);
        self.type_count += 1;
    }

    fn visit_item_union(&mut self, i: &'ast syn::ItemUnion) {
        let decl = Declaration::new(
            DeclarationItem::Union(i.clone()),
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
            self.source_file.clone(),
            self.crate_name.clone(),
            false,
            HashSet::new(),
        );
        let identifier = decl.get_identifier();
        self.declarations.insert(identifier, decl);
        self.union_count += 1;
    }

    fn visit_item(&mut self, i: &'ast syn::Item) {
        syn::visit::visit_item(self, i);
    }
}

pub async fn extract_declarations_from_single_file(
    file_path: &Path,
    rustc_info: &RustcInfo,
    crate_name: &str,
    verbosity: u8,
) -> anyhow::Result<(Vec<Declaration>, Vec<ErrorSample>, FileMetadata)> {
    let mut all_declarations: Vec<Declaration> = Vec::new();
    let mut all_collected_errors: Vec<ErrorSample> = Vec::new();
    let mut file_metadata = FileMetadata::default();

    if verbosity >= 1 {
        println!("Processing single file: {}", file_path.display());
    }

    let file_content = tokio::fs::read_to_string(file_path).await
        .context(format!("Failed to read file: {}", file_path.display()))?;

    match syn::parse_file(&file_content) {
        Ok(file) => {
            for attr in &file.attrs {
                if attr.path().is_ident("feature") {
                    attr.parse_nested_meta(|meta| {
                        if let Some(ident) = meta.path.get_ident() {
                            file_metadata.feature_attributes.insert(ident.to_string());
                        }
                        Ok(())
                    }).ok();
                }
            }

            for item in &file.items {
                match item {
                    syn::Item::Use(item_use) => {
                        let use_statement_str = quote! { #item_use }.to_string();
                        if use_statement_str.contains('"') {
                            if verbosity >= 2 {
                                println!("  [split-expanded-lib] Commenting out invalid global use statement (contains string literal): {}", use_statement_str);
                            }
                            file_metadata.global_uses.insert(format!("// {}", use_statement_str));
                        } else if use_statement_str == "command" || use_statement_str == "arg" || use_statement_str == "doc" {
                            if verbosity >= 2 {
                                println!("  [split-expanded-lib] Commenting out known invalid global use statement: {}", use_statement_str);
                            }
                            file_metadata.global_uses.insert(format!("// {}", use_statement_str));
                        } else {
                            if verbosity >= 2 {
                                println!("  [split-expanded-lib] Global use statement found: {}", use_statement_str);
                            }
                            file_metadata.global_uses.insert(use_statement_str);
                        }
                    }
                    syn::Item::ExternCrate(item_extern_crate) => {
                        file_metadata.extern_crates.insert(item_extern_crate.ident.to_string());
                    }
                    _ => {}
                }
            }

            let mut visitor = DeclsVisitor::new(file_path.to_path_buf(), crate_name.to_string(), verbosity);
            visitor.visit_file(&file);
            all_declarations.extend(visitor.declarations.into_values());
        },
        Err(e) => {
            all_collected_errors.push(ErrorSample {
                file_path: file_path.to_path_buf(),
                rustc_version: rustc_info.version.clone(),
                rustc_host: rustc_info.host.clone(),
                error_message: e.to_string(),
                error_type: "SynParsingFailed".to_string(),
                code_snippet: Some(file_content),
                timestamp: Utc::now(),
                context: None,
            });
        }
    }

    Ok((all_declarations, all_collected_errors, file_metadata))
}