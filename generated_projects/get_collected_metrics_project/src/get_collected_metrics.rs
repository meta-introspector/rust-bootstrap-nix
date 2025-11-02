use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

pub fn get_collected_metrics () -> HashMap < String , FunctionMetrics > { METRICS . lock () . unwrap () . clone () }