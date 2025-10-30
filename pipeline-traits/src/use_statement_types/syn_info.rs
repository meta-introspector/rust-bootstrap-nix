use std::fmt::Debug;

pub trait SynInfo: Send + Sync + Debug {}

#[derive(Debug, Clone)]
pub struct SynDetails {}

impl SynInfo for SynDetails {}
