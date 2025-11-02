use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

fn end_group () { if is_in_gha () { { :: std :: io :: _print (format_args ! ("::endgroup::\n")) ; } ; } }