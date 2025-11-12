pub trait Declaration {
    fn name(&self) -> &str;
    // Add other common methods
}

pub struct FunctionDecl {
    pub name: String,
    // Add other function-specific fields
}

impl Declaration for FunctionDecl {
    fn name(&self) -> &str {
        &self.name
    }
}

pub trait Uses<T: Declaration> {
    fn uses(&self, other: &T) -> bool;
}

pub trait Is<T> {
    fn is_type(&self) -> bool;
}

pub struct DeclarationInstance<T> {
    inner: T,
}

impl<T: Declaration> Declaration for DeclarationInstance<T> {
    fn name(&self) -> &str {
        self.inner.name()
    }
}

// Example implementation of Uses for DeclarationInstance
impl<T: Declaration, U: Declaration> Uses<U> for DeclarationInstance<T> {
    fn uses(&self, _other: &U) -> bool {
        // Placeholder logic
        false
    }
}

// Example implementation of Is for DeclarationInstance
impl<T: Declaration> Is<T> for DeclarationInstance<T> {
    fn is_type(&self) -> bool {
        true // Placeholder logic
    }
}
