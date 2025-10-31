use serde::{Serialize, Deserialize};
# [derive (Debug , Serialize , Deserialize)] pub struct StageSummary { pub total_processed : usize , pub successful : usize , pub failed : usize , }