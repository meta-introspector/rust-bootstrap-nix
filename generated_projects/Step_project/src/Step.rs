use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

pub trait Step { type Output ; const DEFAULT : bool ; const ONLY_HOSTS : bool ; fn should_run (run : ShouldRun) -> ShouldRun ; fn make_run (run : RunConfig) ; fn run (self , builder : & Builder) ; }