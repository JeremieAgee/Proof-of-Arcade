use agee_block::Block;
use agee_ledger::{Balances, SupplyTracker};
use agee_primitives::ClaimKey;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainState {
    pub height: u64,
    pub blocks: Vec<Block>,
    pub balances: Balances,
    pub supply: SupplyTracker,
    pub claimed_floors: std::collections::HashSet<[u8; 32]>,
}

impl ChainState {
    pub fn new() -> Self {
        ChainState {
            height: 0,
            blocks: vec![],
            balances: Balances::new(),
            supply: SupplyTracker::new(),
            claimed_floors: std::collections::HashSet::new(),
        }
    }

    pub fn mark_floor_claimed(&mut self, claim_key: ClaimKey) -> bool {
        self.claimed_floors.insert(claim_key.0)
    }

    pub fn is_floor_claimed(&self, claim_key: &ClaimKey) -> bool {
        self.claimed_floors.contains(&claim_key.0)
    }
}
