use agee_primitives::{
    AccountId, CoinAmount, ChainId, GameId, GameRulesVersion, Hash, ProtocolVersion,
    RunId, Signature, ValidatorId, FloorNumber,
};
use serde::{Deserialize, Serialize};

/// Canonical validation result signed by a validator.
/// Prevents signature reuse across chains, game versions, players, and floors.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorAttestation {
    pub chain_id: ChainId,
    pub protocol_version: ProtocolVersion,

    pub validator_id: ValidatorId,

    pub player: AccountId,
    pub game_id: GameId,
    pub game_rules_version: GameRulesVersion,

    pub run_id: RunId,
    pub floor_number: FloorNumber,

    pub floor_proof_hash: Hash,
    pub calculated_reward: CoinAmount,
    pub reward_epoch: u32,

    pub validated_at_ms: u64,
    pub expires_at_ms: u64,
}

impl ValidatorAttestation {
    pub fn to_canonical_bytes(&self) -> Vec<u8> {
        // TODO: Use deterministic canonical encoding (bincode/postcard)
        serde_json::to_vec(self).unwrap_or_default()
    }

    pub fn domain_separation(&self) -> &'static [u8] {
        b"AGEE_VALIDATOR_ATTESTATION_V1"
    }

    pub fn signed_message(&self) -> Vec<u8> {
        let mut msg = self.domain_separation().to_vec();
        msg.extend(self.to_canonical_bytes());
        msg
    }
}

/// Validator signature with full attestation (for full verification).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorSignature {
    pub validator_id: ValidatorId,
    pub attestation: ValidatorAttestation,
    pub signature: Signature,
}

impl ValidatorSignature {
    pub fn matches_attestation(&self, other: &ValidatorAttestation) -> bool {
        self.attestation.floor_proof_hash == other.floor_proof_hash
            && self.attestation.calculated_reward == other.calculated_reward
            && self.attestation.player == other.player
            && self.attestation.game_id == other.game_id
            && self.attestation.run_id == other.run_id
            && self.attestation.floor_number == other.floor_number
            && self.attestation.game_rules_version == other.game_rules_version
            && self.attestation.reward_epoch == other.reward_epoch
    }
}
