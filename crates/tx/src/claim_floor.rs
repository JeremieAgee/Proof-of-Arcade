use agee_primitives::{AccountId, CoinAmount, GameId, GameRulesVersion, Hash, RunId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedValidatorAttestation {
    pub validator_id: [u8; 32],
    pub attestation_hash: Hash,
    pub signature: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimFloorReward {
    pub player: AccountId,
    pub game_id: GameId,
    pub game_rules_version: GameRulesVersion,
    pub run_id: RunId,
    pub floor_number: u32,
    pub floor_proof_hash: Hash,
    pub claimed_reward: CoinAmount,
    pub attestations: Vec<SignedValidatorAttestation>,
}
