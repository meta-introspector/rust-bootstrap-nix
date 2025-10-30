use std::fmt::Debug;

pub trait CargoInfo: Send + Sync + Debug {}

#[derive(Debug, Clone)]
pub struct CargoDetails {}

impl CargoInfo for CargoDetails {}
