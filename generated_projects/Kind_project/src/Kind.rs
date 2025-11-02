use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

pub enum Kind { Bench , Check , Clippy , Fix , Format , Test , Miri , Suggest , Perf , Build , Doc , Dist , Install , Clean , Run , Setup , Vendor , }