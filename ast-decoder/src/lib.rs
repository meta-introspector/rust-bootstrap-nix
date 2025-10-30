use anyhow::{Context, Result};
use std::path::PathBuf;
use std::pin::Pin;
use std::future::Future;

use syn::{self, visit::{self, Visit}, Item, Visibility};
use quote::quote;

// Removed: use crate::measurement; as it's no longer in the same crate
use pipeline_traits::{PipelineFunctor, ParsedFile};
use pipeline_traits::{AstStatistics, VariableInfo, FunctionInfo};

/// Helper struct to traverse the AST and collect statistics
struct AstVisitor {
    stats: AstStatistics,
    #[allow(dead_code)]
    file_path: PathBuf,
}

impl AstVisitor {
    fn new(file_path: PathBuf) -> Self {
        Self {
            stats: AstStatistics::default(),
            file_path,
        }
    }

    fn increment_node_type_count(&mut self, node_type: &str) {
        *self.stats.node_type_counts.entry(node_type.to_string()).or_insert(0) += 1;
    }
}

impl <'ast> Visit<'ast> for AstVisitor {
    fn visit_item_fn(&mut self, i: &'ast syn::ItemFn) {
        self.increment_node_type_count("function");

        let mut arg_types = Vec::new();
        for arg in &i.sig.inputs {
            if let syn::FnArg::Typed(pat_type) = arg {
                arg_types.push(quote! { #pat_type.ty }.to_string());
            }
        }

        let return_type = if let syn::ReturnType::Type(_, ty) = &i.sig.output {
            quote! { #ty }.to_string()
        } else {
            "()".to_string()
        };

        let visibility_str = match &i.vis {
            Visibility::Public(_) => "public".to_string(),
            Visibility::Restricted(r) => format!("restricted({})", quote!{#r.path}.to_string()),
            Visibility::Inherited => "private".to_string(),
            //_ => "unknown".to_string(), // Catch-all for any other variants
        };

        self.stats.function_definitions.push(FunctionInfo {
            name: i.sig.ident.to_string(),
            visibility: visibility_str,
            arg_count: i.sig.inputs.len() as u32,
            arg_types,
            return_type,
            is_async: i.sig.asyncness.is_some(),
            is_unsafe: i.sig.unsafety.is_some(),
            is_const: i.sig.constness.is_some(),
        });

        visit::visit_item_fn(self, i);
    }

    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        self.increment_node_type_count("struct");
        visit::visit_item_struct(self, i);
    }

    fn visit_item_enum(&mut self, i: &'ast syn::ItemEnum) {
        self.increment_node_type_count("enum");
        visit::visit_item_enum(self, i);
    }

    fn visit_item_trait(&mut self, i: &'ast syn::ItemTrait) {
        self.increment_node_type_count("trait");
        visit::visit_item_trait(self, i);
    }

    fn visit_item_impl(&mut self, i: &'ast syn::ItemImpl) {
        self.increment_node_type_count("impl");
        visit::visit_item_impl(self, i);
    }

    fn visit_item_use(&mut self, i: &'ast syn::ItemUse) {
        self.increment_node_type_count("import");
        // TODO: Extract detailed ImportInfo
        visit::visit_item_use(self, i);
    }

    fn visit_pat_ident(&mut self, i: &'ast syn::PatIdent) {
        // This captures variable declarations like `let x = ...`
        // It's tricky to get the type here without more context, but we can get the name.
        self.increment_node_type_count("variable");
        // TODO: Extract type and mutability more accurately
        self.stats.variable_declarations.push(VariableInfo {
            name: i.ident.to_string(),
            type_name: "unknown".to_string(), // Placeholder
            is_mutable: i.by_ref.is_some() || i.mutability.is_some(),
            scope: "unknown".to_string(), // Placeholder
        });
        visit::visit_pat_ident(self, i);
    }

    // Catch-all for other items to count them
    fn visit_item(&mut self, i: &'ast syn::Item) {
        match i {
            Item::Const(_) => self.increment_node_type_count("const"),
            Item::Static(_) => self.increment_node_type_count("static"),
            Item::Mod(_) => self.increment_node_type_count("module"),
            Item::ForeignMod(_) => self.increment_node_type_count("foreign_module"),
            Item::Macro(_) => self.increment_node_type_count("macro_definition"),
            // Item::Macro2(_) => self.increment_node_type_count("macro_definition2"), // Removed Item::Macro2
            Item::Type(_) => self.increment_node_type_count("type_alias"),
            Item::Union(_) => self.increment_node_type_count("union"),
            _ => self.increment_node_type_count("other_item"), // Fallback for items not explicitly handled
        }
        visit::visit_item(self, i);
    }

    // TODO: Add more specific visitors for expressions, statements, etc.
}

/// Functor to traverse the AST and collect statistics
pub struct AstTraversalFunctor;

impl PipelineFunctor<ParsedFile, AstStatistics> for AstTraversalFunctor {
    fn map<'writer>(
        &'writer self,
        _writer: &'writer mut (impl tokio::io::AsyncWriteExt + Unpin + Send),
        input: ParsedFile,
    ) -> Pin<Box<dyn Future<Output = Result<AstStatistics>> + Send + 'writer>> {
        Box::pin(async move {
            // Removed: measurement::record_function_entry("AstTraversalFunctor::map");
            let ParsedFile(parsed_code, file_path) = input;

            let stats = tokio::task::spawn_blocking(move || -> anyhow::Result<AstStatistics> {
                let ast = syn::parse_file(&parsed_code).context("Failed to parse code into AST for traversal")?;
                let mut visitor = AstVisitor::new(file_path);
                syn::visit::visit_file(&mut visitor, &ast);
                Ok(visitor.stats)
            }).await.context("Blocking task for AST parsing failed")??;

            // Removed: measurement::record_function_exit("AstTraversalFunctor::map");
            Ok(stats)
        })
    }
}
