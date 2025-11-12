pub trait VernacularDeclarationVisitor {
    fn visit_function(&mut self, name: &str);
    fn visit_struct(&mut self, name: &str);
    fn visit_enum(&mut self, name: &str);
    fn visit_trait(&mut self, name: &str);
    fn visit_type_alias(&mut self, name: &str);
    fn visit_union(&mut self, name: &str);
    fn visit_constant(&mut self, name: &str);
    fn visit_static(&mut self, name: &str);
    fn visit_macro(&mut self, name: &str);
    fn visit_module(&mut self, name: &str);
    // Add other declaration types as needed
}
