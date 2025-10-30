use std::fmt::Debug;

pub trait LinuxInfo: Send + Sync + Debug {}

#[derive(Debug, Clone)]
pub struct LinuxDetails {}

impl LinuxInfo for LinuxDetails {}
