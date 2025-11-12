#![feature(prelude_import)]
#![no_std]
#[macro_use]
extern crate std;
#[prelude_import]
use ::std::prelude::rust_2015::*;
pub trait Merge {
    fn merge(&mut self, other: Self, replace: ReplaceOpt);
}

pub enum ReplaceOpt { IgnoreDuplicate, Override, ErrorOnDuplicate, }
#[automatically_derived]
impl ::core::fmt::Debug for ReplaceOpt {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f,
            match self {
                ReplaceOpt::IgnoreDuplicate => "IgnoreDuplicate",
                ReplaceOpt::Override => "Override",
                ReplaceOpt::ErrorOnDuplicate => "ErrorOnDuplicate",
            })
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for ReplaceOpt { }
#[automatically_derived]
impl ::core::cmp::PartialEq for ReplaceOpt {
    #[inline]
    fn eq(&self, other: &ReplaceOpt) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for ReplaceOpt {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::marker::Copy for ReplaceOpt { }
#[automatically_derived]
impl ::core::clone::Clone for ReplaceOpt {
    #[inline]
    fn clone(&self) -> ReplaceOpt { *self }
}

impl<T> Merge for Option<T> where T: Merge + Sized {
    fn merge(&mut self, other: Self, replace: ReplaceOpt) {
        match replace {
            ReplaceOpt::IgnoreDuplicate => {
                if self.is_none() { *self = other; }
            }
            ReplaceOpt::Override => { if other.is_some() { *self = other; } }
            ReplaceOpt::ErrorOnDuplicate => {
                if other.is_some() {
                    if self.is_some() {
                        if false {
                            { ::std::rt::begin_panic("overriding existing option"); }
                        } else {
                            {
                                ::std::io::_eprint(format_args!("overriding existing option\n"));
                            };
                            std::process::exit(2);
                        }
                    } else { *self = other; }
                }
            }
        }
    }
}
