use syn::visit::Visit;
use syn::{ItemConst, ItemStruct};
pub struct Level0DeclsVisitor { pub constants : Vec < ItemConst > , pub numerical_constants : Vec < ItemConst > , pub string_constants : Vec < ItemConst > , pub layer0_structs : Vec < ItemStruct > , pub fn_count : usize , pub struct_count : usize , pub enum_count : usize , pub static_count : usize , pub other_item_count : usize , }