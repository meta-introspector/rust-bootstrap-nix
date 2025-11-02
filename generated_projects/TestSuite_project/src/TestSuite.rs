use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

pub struct TestSuite { pub metadata : TestSuiteMetadata , pub tests : Vec < Test > , }