
#[derive(Debug)]
pub struct TargetSelection(pub String);


impl TargetSelection {
    pub fn from_user(s: &str) -> Self {
        TargetSelection(s.to_string())
    }
}

impl Default for TargetSelection {
    fn default() -> Self {
        TargetSelection::from_user("x86_64-unknown-linux-gnu") // Placeholder default
    }
}

