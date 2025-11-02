use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

pub fn handle_extract_use_statements (_args : & crate :: Args) -> anyhow :: Result < () > { let output_dir = _args . use_statements_output_dir . clone () . ok_or_else (| | :: anyhow :: __private :: must_use ({ let error = :: anyhow :: __private :: format_err (format_args ! ("use_statements_output_dir is required when extract_use_statements is true" ,) ,) ; error })) ? ; { :: std :: io :: _print (format_args ! ("Extracting use statements to: {0}\n" , output_dir . display ()) ,) ; } ; Ok (()) }