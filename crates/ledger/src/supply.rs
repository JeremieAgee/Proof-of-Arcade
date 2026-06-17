use agee_primitives::CoinAmount;
use serde::{Deserialize, Serialize};

const MAX_SUPPLY: u64 = 100_000_000; // 100 million AGEE cap

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplyTracker {
    pub circulating: CoinAmount,
    pub burned: CoinAmount,
}

impl SupplyTracker {
    pub fn new() -> Self {
        SupplyTracker {
            circulating: CoinAmount::new(0),
            burned: CoinAmount::new(0),
        }
    }

    pub fn max_supply() -> CoinAmount {
        CoinAmount::new(MAX_SUPPLY)
    }

    pub fn can_mint(&self, amount: CoinAmount) -> bool {
        self.circulating.0 + amount.0 <= MAX_SUPPLY
    }

    pub fn mint(&mut self, amount: CoinAmount) -> bool {
        if self.can_mint(amount) {
            self.circulating = self.circulating.saturating_add(amount);
            true
        } else {
            false
        }
    }

    pub fn burn(&mut self, amount: CoinAmount) {
        self.circulating = self.circulating.saturating_sub(amount);
        self.burned = self.burned.saturating_add(amount);
    }
}
