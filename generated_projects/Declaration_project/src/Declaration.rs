use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

pub struct Declaration { pub item : DeclarationItem , pub referenced_types : HashSet < String > , pub referenced_functions : HashSet < String > , pub external_identifiers : HashSet < String > , pub gem_identifiers : HashSet < String > , }