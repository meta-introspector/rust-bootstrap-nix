use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

pub fn target_selection_list (s : & str) -> Result < TargetSelectionList , String > { Ok (TargetSelectionList (s . split (',') . filter (| s | ! s . is_empty ()) . map (TargetSelection :: from_user) . collect () ,) ,) }