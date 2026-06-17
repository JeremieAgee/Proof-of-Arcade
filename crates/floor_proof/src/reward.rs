use agee_primitives::CoinAmount;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum EmissionEpoch {
    Epoch1,
    Epoch2,
    Epoch3,
    Epoch4,
    Epoch5,
}

impl EmissionEpoch {
    pub fn multiplier(&self) -> f64 {
        match self {
            EmissionEpoch::Epoch1 => 1.0,
            EmissionEpoch::Epoch2 => 0.5,
            EmissionEpoch::Epoch3 => 0.25,
            EmissionEpoch::Epoch4 => 0.125,
            EmissionEpoch::Epoch5 => 0.0625,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct FloorReward {
    pub base_amount: CoinAmount,
    pub epoch: EmissionEpoch,
}

impl FloorReward {
    pub fn calculated(&self) -> CoinAmount {
        let multiplied = self.base_amount.0 as f64 * self.epoch.multiplier();
        CoinAmount::new(multiplied as u64)
    }
}
