use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

pub struct Install { pub prefix : Option < String > , pub sysconfdir : Option < String > , pub docdir : Option < String > , pub bindir : Option < String > , pub libdir : Option < String > , pub mandir : Option < String > , pub datadir : Option < String > , }