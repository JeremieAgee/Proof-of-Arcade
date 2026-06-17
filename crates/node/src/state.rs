use agee_block::Block;
use agee_consensus::ValidatorSet;
use agee_ledger::{Balances, SupplyTracker};
use agee_primitives::{ClaimKey, Hash};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainState {
    pub height: u64,
    pub blocks: Vec<Block>,
    pub balances: Balances,
    pub supply: SupplyTracker,
    pub claimed_floors: std::collections::HashSet<[u8; 32]>,
    pub validators: ValidatorSet,
}

impl ChainState {
    pub fn new() -> Self {
        ChainState {
            height: 0,
            blocks: vec![],
            balances: Balances::new(),
            supply: SupplyTracker::new(),
            claimed_floors: std::collections::HashSet::new(),
            validators: ValidatorSet::new(1), // Default threshold 1 for v0
        }
    }

    pub fn mark_floor_claimed(&mut self, claim_key: ClaimKey) -> bool {
        self.claimed_floors.insert(claim_key.0)
    }

    pub fn is_floor_claimed(&self, claim_key: &ClaimKey) -> bool {
        self.claimed_floors.contains(&claim_key.0)
    }

    /// Compute the state root hash deterministically.
    pub fn compute_state_root(&self) -> Hash {
        let mut hasher = Sha256::new();

        // Hash height
        hasher.update(self.height.to_le_bytes());

        // Hash supply (circulating + burned)
        hasher.update(self.supply.circulating.0.to_le_bytes());
        hasher.update(self.supply.burned.0.to_le_bytes());

        // Hash claimed floors (sorted for determinism)
        let mut claimed_sorted: Vec<_> = self.claimed_floors.iter().collect();
        claimed_sorted.sort();
        for claim_key in claimed_sorted {
            hasher.update(claim_key);
        }

        let result = hasher.finalize();
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(&result);
        Hash::new(bytes)
    }
}
