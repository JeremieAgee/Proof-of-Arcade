use agee_primitives::{AccountId, CoinAmount, GameRulesVersion, Hash};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimFloorReward {
    pub player: AccountId,
    pub game_id: u32,
    pub game_rules_version: GameRulesVersion,
    pub run_id: u64,
    pub floor_number: u32,
    pub floor_proof_hash: Hash,
    pub claimed_amount: CoinAmount,
    pub validator_signatures: Vec<Vec<u8>>,
}
