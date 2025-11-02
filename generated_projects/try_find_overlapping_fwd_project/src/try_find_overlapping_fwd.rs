use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

# [inline (never)] fn try_find_overlapping_fwd < A : Automaton + ? Sized > (aut : & A , input : & Input < '_ > , state : & mut OverlappingState ,) -> Result < () , MatchError > { state . mat = None ; if input . is_done () { return Ok (()) ; } if aut . prefilter () . is_some () && ! input . get_anchored () . is_anchored () { let pre = aut . prefilter () . unwrap () ; try_find_overlapping_fwd_imp (aut , input , Some (pre) , state) } else { try_find_overlapping_fwd_imp (aut , input , None , state) } }