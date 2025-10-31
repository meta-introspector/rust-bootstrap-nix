use syn::{visit::Visit, ItemConst, File, ItemFn, ItemStruct, ItemEnum, ItemStatic};


pub struct DeclsVisitor {
    pub constants: Vec<ItemConst>,
    pub numerical_constants: Vec<ItemConst>,
    pub string_constants: Vec<ItemConst>,
    pub all_structs: Vec<ItemStruct>,
    pub fn_count: usize,
    pub struct_count: usize,
    pub enum_count: usize,
    pub static_count: usize,
    pub other_item_count: usize,
}

impl DeclsVisitor {
    pub fn new() -> Self {
        DeclsVisitor {
            constants: Vec::new(),
            numerical_constants: Vec::new(),
            string_constants: Vec::new(),
            all_structs: Vec::new(),
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

impl<'ast> Visit<'ast> for DeclsVisitor {
    fn visit_item_const(&mut self, i: &'ast ItemConst) {
        self.constants.push(i.clone());
        // Check if the constant's expression is a literal number or string
        if let syn::Expr::Lit(expr_lit) = &*i.expr {
            match &expr_lit.lit {
                syn::Lit::Int(_) | syn::Lit::Float(_) => {
                    self.numerical_constants.push(i.clone());
                },
                syn::Lit::Str(_) => {
                    self.string_constants.push(i.clone());
                },
                _ => {},
            }
        }
        syn::visit::visit_item_const(self, i);
    }

    fn visit_item_fn(&mut self, i: &'ast ItemFn) {
        self.fn_count += 1;
        syn::visit::visit_item_fn(self, i);
    }

    fn visit_item_struct(&mut self, i: &'ast ItemStruct) {
        self.struct_count += 1;
        self.all_structs.push(i.clone());
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

