use build_helper::prelude::*;
use config_macros::define_config;
define_config! {
    #[derive(Deserialize)]
    struct Install {
    prefix : Option < String > = "prefix", sysconfdir : Option < String > = "sysconfdir",
    docdir : Option < String > = "docdir", bindir : Option < String > = "bindir", libdir
    : Option < String > = "libdir", mandir : Option < String > = "mandir", datadir :
    Option < String > = "datadir", }
}
