use agee_primitives::ValidatorId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorInfo {
    pub validator_id: ValidatorId,
    pub public_key: [u8; 32],
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorSet {
    pub active: HashMap<ValidatorId, ValidatorInfo>,
    pub threshold: u32,
}

impl ValidatorSet {
    pub fn new(threshold: u32) -> Self {
        ValidatorSet {
            active: HashMap::new(),
            threshold,
        }
    }

    pub fn add_validator(&mut self, info: ValidatorInfo) {
        self.active.insert(info.validator_id, info);
    }

    pub fn is_active(&self, validator_id: ValidatorId) -> bool {
        self.active
            .get(&validator_id)
            .map(|info| info.active)
            .unwrap_or(false)
    }

    pub fn get_info(&self, validator_id: ValidatorId) -> Option<&ValidatorInfo> {
        self.active.get(&validator_id)
    }

    pub fn size(&self) -> usize {
        self.active.len()
    }

    pub fn active_count(&self) -> usize {
        self.active.values().filter(|v| v.active).count()
    }
}
