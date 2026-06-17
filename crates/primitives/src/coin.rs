use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CoinAmount(pub u64);

impl CoinAmount {
    pub fn new(amount: u64) -> Self {
        CoinAmount(amount)
    }

    pub fn saturating_add(self, other: CoinAmount) -> CoinAmount {
        CoinAmount(self.0.saturating_add(other.0))
    }

    pub fn saturating_sub(self, other: CoinAmount) -> CoinAmount {
        CoinAmount(self.0.saturating_sub(other.0))
    }
}
