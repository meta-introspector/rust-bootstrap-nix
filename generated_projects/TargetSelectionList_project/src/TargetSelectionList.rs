use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

# [doc = " Newtype over `Vec<TargetSelection>` so we can implement custom parsing logic"] pub struct TargetSelectionList (pub Vec < TargetSelection >) ;