use split_expanded_lib::{Declaration, ResolvedDependency};


#[derive(Debug)]
pub enum ValidationError {
    DependencyResolutionError(String),
    MissingCargoInfo(String),
    UnresolvedDependency(ResolvedDependency),
    // Add more specific validation errors as needed
}

pub trait DeclarationValidator {
    fn validate(&self, declaration: &Declaration) -> Result<(), ValidationError>;
}

pub struct DependencyValidator;

impl DeclarationValidator for DependencyValidator {
    fn validate(&self, declaration: &Declaration) -> Result<(), ValidationError> {
        for _dep in &declaration.referenced_types {
        }
        Ok(())
    }
}
