pub mod prelude;
mod prelude; // Declare prelude module
pub use prelude::*; // Re-export prelude contents

pub use bootstrap_config_builder_core::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DocTests {
    Yes,
    No,
    Only,
}

