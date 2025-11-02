use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

pub struct Dist { pub sign_folder : Option < String > , pub upload_addr : Option < String > , pub src_tarball : Option < bool > , pub compression_formats : Option < Vec < String > > , pub compression_profile : Option < String > , pub include_mingw_linker : Option < bool > , pub vendor : Option < bool > , }