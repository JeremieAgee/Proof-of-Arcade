use agee_primitives::{ClaimKey, CoinAmount};
use serde::{Deserialize, Serialize};

/// Result of verifying a claim's attestations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifiedClaim {
    pub claim_key: ClaimKey,
    pub reward: CoinAmount,
    pub reward_epoch: u32,
}

impl VerifiedClaim {
    pub fn new(claim_key: ClaimKey, reward: CoinAmount, reward_epoch: u32) -> Self {
        VerifiedClaim {
            claim_key,
            reward,
            reward_epoch,
        }
    }
}
