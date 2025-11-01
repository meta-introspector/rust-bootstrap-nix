use build_helper::prelude::*;
define_config! {
    struct Install {
    prefix : Option < String > = "prefix", sysconfdir : Option < String > = "sysconfdir",
    docdir : Option < String > = "docdir", bindir : Option < String > = "bindir", libdir
    : Option < String > = "libdir", mandir : Option < String > = "mandir", datadir :
    Option < String > = "datadir", }
}
