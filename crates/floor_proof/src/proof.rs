use agee_primitives::{AccountId, GameRulesVersion, Hash};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FloorProof {
    pub run_id: u64,
    pub game_id: u32,
    pub game_rules_version: GameRulesVersion,
    pub player_id: AccountId,
    pub wallet_address: AccountId,
    pub floor_number: u32,
    pub floor_seed: Hash,
    pub start_time: u64,
    pub end_time: u64,
    pub claimed_coin_amount: u64,
    pub completion_state: String,
    pub proof_hash: Hash,
}
