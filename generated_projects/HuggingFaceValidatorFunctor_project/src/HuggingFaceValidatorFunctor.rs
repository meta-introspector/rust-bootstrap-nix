use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

pub struct HuggingFaceValidatorFunctor { pub args : crate :: Args , pub hf_validator_path : Option < PathBuf > , }