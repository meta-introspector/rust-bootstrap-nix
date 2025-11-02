use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

# [doc = " A set of state identifiers used to avoid revisiting the same state multiple"] # [doc = " times when filling in failure transitions."] # [doc = ""] # [doc = " This set has an \"inert\" and an \"active\" mode. When inert, the set never"] # [doc = " stores anything and always returns `false` for every member test. This is"] # [doc = " useful to avoid the performance and memory overhead of maintaining this"] # [doc = " set when it is not needed."] struct QueuedSet { set : Option < BTreeSet < StateID > > , }