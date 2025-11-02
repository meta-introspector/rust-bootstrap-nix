use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

pub struct InspectFunctor < 'a , T : Debug > { label : & 'a str , _phantom : std :: marker :: PhantomData < T > , }