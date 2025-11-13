use crate::prelude::*;
use std::ffi::OsString;
use std::ffi::OsStr;
#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct DropBomb {
    command: OsString,
    defused: bool,
    armed_location: panic::Location<'static>,
}
impl DropBomb {
    /// Arm a [`DropBomb`]. If the value is dropped without being [`defused`][Self::defused], then
    /// it will panic. It is expected that the command wrapper uses `#[track_caller]` to help
    /// propagate the caller location.
    #[track_caller]
    pub fn arm<S: AsRef<OsStr>>(command: S) -> DropBomb {
        DropBomb {
            command: command.as_ref().into(),
            defused: false,
            armed_location: *panic::Location::caller(),
        }
    }
    pub fn get_created_location(&self) -> panic::Location<'static> {
        self.armed_location
    }
    /// Defuse the [`DropBomb`]. This will prevent the drop bomb from panicking when dropped.
    pub fn defuse(&mut self) {
        self.defused = true;
    }
}
impl Drop for DropBomb {
    fn drop(&mut self) {
        if !self.defused && !std::thread::panicking() {
            panic!(
                "command constructed at `{}` was dropped without being executed: `{}`",
                self.armed_location, self.command.to_string_lossy()
            )
        }
    }
}
