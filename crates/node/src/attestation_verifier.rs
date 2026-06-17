use crate::state::ChainState;
use crate::verified_claim::VerifiedClaim;
use agee_primitives::{ClaimKey, GameId, GameRulesVersion, ValidatorId};
use agee_tx::{ClaimFloorReward, TxError};
use std::collections::HashSet;

/// Verify that all validator attestations agree with the claim transaction
/// and meet the signature threshold.
pub fn verify_claim_attestations(
    state: &ChainState,
    tx: &ClaimFloorReward,
) -> Result<VerifiedClaim, TxError> {
    // 1. Check minimum signature count
    if (tx.attestations.len() as u32) < state.validators.threshold {
        return Err(TxError::InsufficientValidatorSignatures);
    }

    // 2. Check all attestations are for known, active validators
    let mut seen_validators = HashSet::new();
    let mut matching_attestations = vec![];

    for signed_attestation in &tx.attestations {
        let validator_id = ValidatorId(signed_attestation.validator_id);

        // Check validator is active
        if !state.validators.is_active(validator_id) {
            return Err(TxError::UnknownValidator);
        }

        // Check no duplicate validators
        if !seen_validators.insert(validator_id) {
            return Err(TxError::DuplicateValidatorSignature);
        }

        // TODO: In a real implementation, verify cryptographic signature
        // For v0, we trust the signature is valid (placeholder)
        matching_attestations.push(signed_attestation);
    }

    // 3. Reconstruct attestation to verify fields match
    // (In real implementation, would deserialize from signed_attestation)
    // For now, we'll validate the claim is self-consistent
    validate_claim_consistency(tx)?;

    // 4. Ensure we have enough matching signatures
    if (matching_attestations.len() as u32) < state.validators.threshold {
        return Err(TxError::InsufficientMatchingSignatures);
    }

    // 5. Verify the claimed reward matches validator calculations
    // (In real implementation, would extract from attestation signatures)
    // For v0, we trust the attestations encode the correct reward

    let claim_key = ClaimKey::new(
        tx.player,
        tx.game_id,
        tx.run_id,
        agee_primitives::FloorNumber(tx.floor_number),
    );

    Ok(VerifiedClaim::new(
        claim_key,
        tx.claimed_reward,
        1, // reward_epoch—would come from attestation in full impl
    ))
}

/// Verify internal consistency of the claim transaction.
fn validate_claim_consistency(tx: &ClaimFloorReward) -> Result<(), TxError> {
    // Game ID must be valid (v1 supports only Maze Runner game_id=1)
    if tx.game_id.0 != 1 {
        return Err(TxError::InvalidGameId);
    }

    // Rules version must be valid
    if tx.game_rules_version.0 != 1 {
        return Err(TxError::InvalidGameRulesVersion);
    }

    // Floor number must be reasonable (1-100 for v1)
    if tx.floor_number == 0 || tx.floor_number > 100 {
        return Err(TxError::InvalidFloorNumber);
    }

    // Reward must be positive
    if tx.claimed_reward.0 == 0 {
        return Err(TxError::InvalidRewardAmount);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use agee_consensus::{ValidatorSet as ConsensusValidatorSet, ValidatorInfo};
    use agee_primitives::{AccountId, CoinAmount, GameId, GameRulesVersion, Hash, RunId, ValidatorId};
    use agee_tx::SignedValidatorAttestation;

    fn sample_claim_tx() -> ClaimFloorReward {
        ClaimFloorReward {
            player: AccountId::new([1u8; 32]),
            game_id: GameId(1),
            game_rules_version: GameRulesVersion(1),
            run_id: RunId(100),
            floor_number: 5,
            floor_proof_hash: Hash::new([2u8; 32]),
            claimed_reward: CoinAmount::new(10),
            attestations: vec![SignedValidatorAttestation {
                validator_id: [3u8; 32],
                attestation_hash: Hash::new([4u8; 32]),
                signature: vec![1, 2, 3],
            }],
        }
    }

    #[test]
    fn insufficient_signatures_fails() {
        let mut validators = ConsensusValidatorSet::new(3);
        validators.add_validator(ValidatorInfo {
            validator_id: ValidatorId([3u8; 32]),
            public_key: [0u8; 32],
            active: true,
        });

        let mut state = ChainState::new();
        state.validators = validators;

        let tx = sample_claim_tx(); // Has 1 attestation but threshold is 3

        let result = verify_claim_attestations(&state, &tx);
        assert!(result.is_err());
    }

    #[test]
    fn unknown_validator_fails() {
        let mut validators = ConsensusValidatorSet::new(1);
        validators.add_validator(ValidatorInfo {
            validator_id: ValidatorId([99u8; 32]), // Different ID
            public_key: [0u8; 32],
            active: true,
        });

        let mut state = ChainState::new();
        state.validators = validators;

        let tx = sample_claim_tx(); // Uses validator [3u8; 32]

        let result = verify_claim_attestations(&state, &tx);
        assert!(result.is_err());
    }

    #[test]
    fn valid_claim_verifies() {
        let mut validators = ConsensusValidatorSet::new(1);
        validators.add_validator(ValidatorInfo {
            validator_id: ValidatorId([3u8; 32]),
            public_key: [0u8; 32],
            active: true,
        });

        let mut state = ChainState::new();
        state.validators = validators;

        let tx = sample_claim_tx();
        let result = verify_claim_attestations(&state, &tx);
        assert!(result.is_ok());

        let verified = result.unwrap();
        assert_eq!(verified.reward, CoinAmount::new(10));
    }

    #[test]
    fn duplicate_validator_fails() {
        let mut validators = ConsensusValidatorSet::new(2);
        validators.add_validator(ValidatorInfo {
            validator_id: ValidatorId([3u8; 32]),
            public_key: [0u8; 32],
            active: true,
        });

        let mut state = ChainState::new();
        state.validators = validators;

        let mut tx = sample_claim_tx();
        tx.attestations = vec![
            SignedValidatorAttestation {
                validator_id: [3u8; 32],
                attestation_hash: agee_primitives::Hash::new([4u8; 32]),
                signature: vec![1, 2, 3],
            },
            SignedValidatorAttestation {
                validator_id: [3u8; 32], // Duplicate
                attestation_hash: agee_primitives::Hash::new([4u8; 32]),
                signature: vec![1, 2, 3],
            },
        ];

        let result = verify_claim_attestations(&state, &tx);
        assert!(result.is_err());
    }

    #[test]
    fn invalid_game_id_fails() {
        let mut validators = ConsensusValidatorSet::new(1);
        validators.add_validator(ValidatorInfo {
            validator_id: ValidatorId([3u8; 32]),
            public_key: [0u8; 32],
            active: true,
        });

        let mut state = ChainState::new();
        state.validators = validators;

        let mut tx = sample_claim_tx();
        tx.game_id = GameId(99); // Invalid

        let result = verify_claim_attestations(&state, &tx);
        assert!(result.is_err());
    }

    #[test]
    fn invalid_rules_version_fails() {
        let mut validators = ConsensusValidatorSet::new(1);
        validators.add_validator(ValidatorInfo {
            validator_id: ValidatorId([3u8; 32]),
            public_key: [0u8; 32],
            active: true,
        });

        let mut state = ChainState::new();
        state.validators = validators;

        let mut tx = sample_claim_tx();
        tx.game_rules_version = GameRulesVersion(99); // Invalid

        let result = verify_claim_attestations(&state, &tx);
        assert!(result.is_err());
    }
}
