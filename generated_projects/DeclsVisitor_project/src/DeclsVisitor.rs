use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

pub struct DeclsVisitor { pub declarations : Vec < Declaration > , pub fn_count : usize , pub struct_count : usize , pub enum_count : usize , pub static_count : usize , pub other_item_count : usize , pub referenced_types : HashSet < String > , pub referenced_functions : HashSet < String > , pub external_identifiers : HashSet < String > , pub current_gem_identifiers : HashSet < String > , pub gem_identifiers : HashMap < String , String > , }