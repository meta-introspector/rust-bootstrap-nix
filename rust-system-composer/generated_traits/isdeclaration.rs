pub trait IsDeclaration {
    fn get_declaration_name(&self) -> &'static str;
}
