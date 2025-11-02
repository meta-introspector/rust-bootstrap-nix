use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

pub fn fail (s : & str) -> ! { { :: std :: io :: _eprint (format_args ! ("\n\n{0}\n\n\n" , s)) ; } ; detail_exit (1 , false) ; }