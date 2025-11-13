macro_rules! forward {
    ( $( $fn:ident( $($param:ident: $ty:ty),* ) $( -> $ret:ty)? ),+ $(,)? ) => {
        impl Build {
            $( fn $fn(&self, $($param: $ty),* ) $( -> $ret)? {
                self.config.$fn( $($param),* )
            } )+
        }
    }
}
pub(crate) use forward;
