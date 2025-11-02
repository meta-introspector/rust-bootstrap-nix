use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

fn start_group (name : impl std :: fmt :: Display) { if is_in_gha () { { :: std :: io :: _print (format_args ! ("::group::{0}\n" , name)) ; } ; } else { { :: std :: io :: _print (format_args ! ("{0}\n" , name)) ; } } }