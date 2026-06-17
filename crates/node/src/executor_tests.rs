#[cfg(test)]
mod tests {
    use crate::executor::execute_claim_floor_reward;
    use crate::state::ChainState;
    use agee_consensus::{ValidatorInfo, ValidatorSet};
    use agee_primitives::{AccountId, CoinAmount, GameId, GameRulesVersion, RunId, ValidatorId};
    use agee_tx::{ClaimFloorReward, SignedValidatorAttestation};

    fn sample_claim_tx() -> ClaimFloorReward {
        ClaimFloorReward {
            player: AccountId::new([1u8; 32]),
            game_id: GameId(1),
            game_rules_version: GameRulesVersion(1),
            run_id: RunId(100),
            floor_number: 5,
            floor_proof_hash: agee_primitives::Hash::new([2u8; 32]),
            claimed_reward: CoinAmount::new(10),
            attestations: vec![SignedValidatorAttestation {
                validator_id: [3u8; 32],
                attestation_hash: agee_primitives::Hash::new([4u8; 32]),
                signature: vec![1, 2, 3],
            }],
        }
    }

    fn setup_state_with_validator() -> ChainState {
        let mut state = ChainState::new();
        state.validators = ValidatorSet::new(1);
        state.validators.add_validator(ValidatorInfo {
            validator_id: ValidatorId([3u8; 32]),
            public_key: [0u8; 32],
            active: true,
        });
        state
    }

    #[test]
    fn valid_claim_mints_coins() {
        let mut state = setup_state_with_validator();
        let tx = sample_claim_tx();

        let result = execute_claim_floor_reward(&mut state, &tx);
        assert!(result.is_ok());

        let balance = state.balances.get(tx.player).unwrap();
        assert_eq!(balance.mintable_coins(), CoinAmount::new(10));
        assert_eq!(state.supply.circulating, CoinAmount::new(10));
    }

    #[test]
    fn duplicate_claim_fails() {
        let mut state = setup_state_with_validator();
        let tx = sample_claim_tx();

        assert!(execute_claim_floor_reward(&mut state, &tx).is_ok());

        let result = execute_claim_floor_reward(&mut state, &tx);
        assert!(result.is_err());
    }

    #[test]
    fn claim_over_max_supply_fails() {
        let mut state = setup_state_with_validator();
        let max_supply = agee_ledger::SupplyTracker::max_supply();
        let mut tx = sample_claim_tx();
        tx.claimed_reward = CoinAmount::new(max_supply.0 + 1);

        let result = execute_claim_floor_reward(&mut state, &tx);
        assert!(result.is_err());
        assert_eq!(state.supply.circulating, CoinAmount::new(0));
    }

    #[test]
    fn no_attestations_fails() {
        let mut state = setup_state_with_validator();
        let mut tx = sample_claim_tx();
        tx.attestations = vec![];

        let result = execute_claim_floor_reward(&mut state, &tx);
        assert!(result.is_err());
    }

    #[test]
    fn unknown_validator_fails() {
        let mut state = setup_state_with_validator();
        let mut tx = sample_claim_tx();
        tx.attestations[0].validator_id = [99u8; 32]; // Unknown validator

        let result = execute_claim_floor_reward(&mut state, &tx);
        assert!(result.is_err());
    }

    #[test]
    fn invalid_game_id_fails() {
        let mut state = setup_state_with_validator();
        let mut tx = sample_claim_tx();
        tx.game_id = GameId(99); // Invalid game

        let result = execute_claim_floor_reward(&mut state, &tx);
        assert!(result.is_err());
    }
}
