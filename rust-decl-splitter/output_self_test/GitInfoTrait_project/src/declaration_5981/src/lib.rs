#![feature(panic_internals)]
#![feature(print_internals)]

use prelude::*;

pub trait GitInfoTrait : Send + Sync + Debug { fn git_repo (& self) -> Option < & str > ; fn git_path (& self) -> Option < & str > ; fn our_fork_github (& self) -> Option < & str > ; fn our_branch (& self) -> Option < & str > ; }