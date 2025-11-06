use anyhow :: Result ;
use std :: path :: PathBuf ;

pub mod syn_impl ;
 pub mod cargo_impl ;
 pub mod git_impl ;
 pub mod rustc_impl ;
 pub mod io_impl ;
pub trait SynInterface {
    fn parse_file < > (&self, content : & str) -> Result < syn :: File > ;    
    fn parse_str < T : syn :: parse :: Parse > (&self,s : & str) -> Result < T > ;
}
pub trait CargoInterface {
    fn run_command < > (&self, args : & [& str] , current_dir : Option <& PathBuf >) -> Result < String > ;
    fn metadata < > (&self,manifest_path : & PathBuf) -> Result < cargo_metadata :: Metadata > ;
    fn expand_macro < > (&self,manifest_path : & PathBuf , lib_name : & str) -> Result < String > ;
}
pub trait GitInterface {
    fn run_command < > (&self,args : & [& str] , current_dir : Option <& PathBuf >) -> Result < String > ;
    fn get_current_branch < > (&self,current_dir : Option <& PathBuf >) -> Result < String > ;
    fn get_last_commit_hash < > (&self,current_dir : Option <& PathBuf >) -> Result < String > ;
}
pub trait RustcInterface {
    fn run_command < > (&self,args : & [& str] , current_dir : Option <& PathBuf >) -> Result < String > ;
    fn get_version_verbose < > (&self) -> Result < String > ;
    fn get_sysroot < > (&self) -> Result < PathBuf > ;
}
pub trait IoInterface {
    fn read_file < > (&self,path : & PathBuf) -> impl std :: future :: Future < Output = Result < String >> + Send ;
    fn write_file < > (&self, path : & PathBuf , content : & str) -> impl std :: future :: Future < Output = Result < () >> + Send ;
    fn create_dir_all < > (&self,path : & PathBuf) -> impl std :: future :: Future < Output = Result < () >> + Send ;
    fn remove_dir_all < > (&self,path : & PathBuf) -> impl std :: future :: Future < Output = Result < () >> + Send ;
    fn path_exists < > (&self,path : & PathBuf) -> impl std :: future :: Future < Output = bool > + Send ;
    fn run_command < > (&self,program : & str , args : & [& str] , current_dir : Option <& PathBuf >) -> impl std :: future :: Future < Output = Result < String >> + Send ;
}

pub struct ExternalInterfaceGateway {
    pub syninterface : Box < dyn SynInterface + Send + Sync > ,
    pub cargointerface : Box < dyn CargoInterface + Send + Sync > ,
    pub gitinterface : Box < dyn GitInterface + Send + Sync > ,
    pub rustcinterface : Box < dyn RustcInterface + Send + Sync > ,
    pub iointerface : Box < dyn IoInterface + Send + Sync > ,
}

impl Default for ExternalInterfaceGateway {
    fn default () -> Self {
	Self { syninterface : Box :: new (syn_impl :: SynInterfaceImpl) ,
	       cargointerface : Box :: new (cargo_impl :: CargoInterfaceImpl) ,
	       gitinterface : Box :: new (git_impl :: GitInterfaceImpl) ,
	       rustcinterface : Box :: new (rustc_impl :: RustcInterfaceImpl) ,
	       iointerface : Box :: new (io_impl :: IoInterfaceImpl)
	}
    }
}
