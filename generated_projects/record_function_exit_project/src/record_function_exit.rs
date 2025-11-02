use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

pub fn record_function_exit (function_name : & str) { let mut metrics = METRICS . lock () . unwrap () ; if let Some (entry) = metrics . get_mut (function_name) { entry . end_time = Some (Instant :: now ()) ; let duration = entry . end_time . unwrap () . duration_since (entry . start_time) ; entry . duration_micros = Some (duration . as_micros ()) ; } }