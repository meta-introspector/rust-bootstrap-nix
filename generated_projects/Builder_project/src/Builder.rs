use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

pub struct Builder < 'a > { pub top_stage : u32 , pub _phantom : std :: marker :: PhantomData < & 'a () > , }