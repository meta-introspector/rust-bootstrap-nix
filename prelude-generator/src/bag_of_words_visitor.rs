use syn::{visit::Visit, ItemConst, ItemFn, ItemStruct, ItemEnum, ItemStatic, Ident};
use std::collections::HashMap;
use regex::Regex;
use once_cell::sync::Lazy;

// Lazy static regex for splitting identifiers
static RE_SPLIT_IDENT: Lazy<Regex> = Lazy::new(|| {
    // Splits on non-alphanumeric, or camelCase transitions, or leading/trailing underscores
    Regex::new(r"[^a-zA-Z0-9]+|(?<=[a-z])(?=[A-Z])|^_|_$").unwrap()
});

pub fn tokenize_ident_to_subwords(ident_str: &str) -> Vec<String> {
    RE_SPLIT_IDENT.split(ident_str)
        .filter(|s| !s.is_empty())
        .map(|s| s.to_lowercase())
        .collect()
}

pub struct BagOfWordsVisitor {
    pub bag_of_words: HashMap<String, usize>,
}

impl BagOfWordsVisitor {
    pub fn new() -> Self {
        BagOfWordsVisitor {
            bag_of_words: HashMap::new(),
        }
    }

    pub fn extract_from_file(file: &syn::File) -> Self {
        let mut visitor = Self::new();
        visitor.visit_file(file);
        visitor
    }

    fn add_ident_to_bag(&mut self, ident: &Ident) {
        for subword in tokenize_ident_to_subwords(&ident.to_string()) {
            *self.bag_of_words.entry(subword).or_insert(0) += 1;
        }
    }
}

impl<'ast> Visit<'ast> for BagOfWordsVisitor {
    fn visit_item_const(&mut self, i: &'ast ItemConst) {
        self.add_ident_to_bag(&i.ident);
        syn::visit::visit_item_const(self, i);
    }

    fn visit_item_fn(&mut self, i: &'ast ItemFn) {
        self.add_ident_to_bag(&i.sig.ident);
        syn::visit::visit_item_fn(self, i);
    }

    fn visit_item_struct(&mut self, i: &'ast ItemStruct) {
        self.add_ident_to_bag(&i.ident);
        // Also visit fields
        for field in &i.fields {
            if let Some(ident) = &field.ident {
                self.add_ident_to_bag(ident);
            }
        }
        syn::visit::visit_item_struct(self, i);
    }

    fn visit_item_enum(&mut self, i: &'ast ItemEnum) {
        self.add_ident_to_bag(&i.ident);
        // Also visit variants
        for variant in &i.variants {
            self.add_ident_to_bag(&variant.ident);
            for field in &variant.fields {
                if let Some(ident) = &field.ident {
                    self.add_ident_to_bag(ident);
                }
            }
        }
        syn::visit::visit_item_enum(self, i);
    }

    fn visit_item_static(&mut self, i: &'ast ItemStatic) {
        self.add_ident_to_bag(&i.ident);
        syn::visit::visit_item_static(self, i);
    }

    // You can add more visit methods for other items if you want to extract more identifiers
    // e.g., trait names, impl names, macro names, etc.
}
