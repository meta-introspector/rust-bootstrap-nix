use syn::{ItemFn, ItemStruct, ItemEnum, ItemTrait, ItemType, ItemUnion, ItemConst, ItemStatic, ItemMacro, ItemMod};

pub trait DeclarationVisitor<'ast> {
    fn visit_fn(&mut self, i: &'ast ItemFn);
    fn visit_struct(&mut self, i: &'ast ItemStruct);
    fn visit_enum(&mut self, i: &'ast ItemEnum);
    fn visit_trait(&mut self, i: &'ast ItemTrait);
    fn visit_type(&mut self, i: &'ast ItemType);
    fn visit_union(&mut self, i: &'ast ItemUnion);
    fn visit_const(&mut self, i: &'ast ItemConst);
    fn visit_static(&mut self, i: &'ast ItemStatic);
    fn visit_macro(&mut self, i: &'ast ItemMacro);
    fn visit_mod(&mut self, i: &'ast ItemMod);
    // Add other declaration types as needed
}
