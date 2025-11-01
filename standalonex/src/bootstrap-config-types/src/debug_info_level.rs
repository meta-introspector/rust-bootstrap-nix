use std::fmt;
use serde::{Deserialize, Deserializer};
use build_helper::prelude::*;
#[derive(Copy, Clone, Default, Debug, Eq, PartialEq)]
pub enum DebuginfoLevel {
    #[default]
    None,
    LineDirectivesOnly,
    LineTablesOnly,
    Limited,
    Full,
}
impl<'de> Deserialize<'de> for DebuginfoLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(
            match Deserialize::deserialize(deserializer)? {
                StringOrInt::String(s) if s == "none" => DebuginfoLevel::None,
                StringOrInt::Int(0) => DebuginfoLevel::None,
                StringOrInt::String(s) if s == "line-directives-only" => {
                    DebuginfoLevel::LineDirectivesOnly
                }
                StringOrInt::String(s) if s == "line-tables-only" => {
                    DebuginfoLevel::LineTablesOnly
                }
                StringOrInt::String(s) if s == "limited" => DebuginfoLevel::Limited,
                StringOrInt::Int(1) => DebuginfoLevel::Limited,
                StringOrInt::String(s) if s == "full" => DebuginfoLevel::Full,
                StringOrInt::Int(2) => DebuginfoLevel::Full,
                StringOrInt::Int(n) => {
                    let other = serde::de::Unexpected::Signed(n);
                    return Err(D::Error::invalid_value(other, &"expected 0, 1, or 2"));
                }
                StringOrInt::String(s) => {
                    let other = serde::de::Unexpected::Str(&s);
                    return Err(
                        D::Error::invalid_value(
                            other,
                            &"expected none, line-tables-only, limited, or full",
                        ),
                    );
                }
            },
        )
    }
}
/// Suitable for passing to `-C debuginfo`
impl Display for DebuginfoLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use DebuginfoLevel::*;
        f.write_str(
            match self {
                None => "0",
                LineDirectivesOnly => "line-directives-only",
                LineTablesOnly => "line-tables-only",
                Limited => "1",
                Full => "2",
            },
        )
    }
}
