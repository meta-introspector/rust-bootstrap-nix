pub trait Merge {
    fn merge(&mut self, other: Self, replace: ReplaceOpt);
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ReplaceOpt {
    IgnoreDuplicate,
    Override,
    ErrorOnDuplicate,
}

impl<T> Merge for Option<T>
where
    T: Merge + Sized,
{
    fn merge(&mut self, other: Self, replace: ReplaceOpt) {
        match replace {
            ReplaceOpt::IgnoreDuplicate => {
                if self.is_none() {
                    *self = other;
                }
            }
            ReplaceOpt::Override => {
                if other.is_some() {
                    *self = other;
                }
            }
            ReplaceOpt::ErrorOnDuplicate => {
                if other.is_some() {
                    if self.is_some() {
                        if cfg!(test) {
                            panic!("overriding existing option")
                        } else {
                            eprintln!("overriding existing option");
                            std::process::exit(2);
                        }
                    } else {
                        *self = other;
                    }
                }
            }
        }
    }
}
