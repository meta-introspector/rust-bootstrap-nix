use crate::prelude::*;
define_config! {
    #[doc = " TOML representation of various global install decisions."] struct Install {
    prefix : Option < String > = "prefix", sysconfdir : Option < String > = "sysconfdir",
    docdir : Option < String > = "docdir", bindir : Option < String > = "bindir", libdir
    : Option < String > = "libdir", mandir : Option < String > = "mandir", datadir :
    Option < String > = "datadir", }
}
