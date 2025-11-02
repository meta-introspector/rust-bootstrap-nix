use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

fn contains_complex_type_in_type (ty : & Type) -> bool { match ty { Type :: Path (type_path) => { for segment in type_path . path . segments . iter () { let ident_str = segment . ident . to_string () ; if is_complex_type (& ident_str) { return true ; } if let PathArguments :: AngleBracketed (angle_args) = & segment . arguments { for arg in angle_args . args . iter () { if let GenericArgument :: Type (inner_ty) = arg { if contains_complex_type_in_type (inner_ty) { return true ; } } } } } false } _ => false , } }