use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

pub struct FunctionMetrics { # [serde (skip)] pub start_time : Instant , # [serde (skip)] pub end_time : Option < Instant > , pub duration_micros : Option < u128 > , pub call_count : u64 , }