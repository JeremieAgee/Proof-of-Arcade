#[cfg(test)]
mod tests {
    use crate::runtime::{NodeRuntime, NodeConfig};
    use agee_consensus::{ValidatorInfo, ValidatorSet};
    use agee_primitives::{AccountId, CoinAmount, GameId, GameRulesVersion, Hash, RunId, ValidatorId};
    use agee_tx::{ClaimFloorReward, SignedValidatorAttestation};

    fn sample_claim_tx(player: AccountId) -> ClaimFloorReward {
        ClaimFloorReward {
            player,
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

    #[tokio::test]
    async fn node_starts_with_genesis_state() {
        let config = NodeConfig::default();
        let runtime = NodeRuntime::new(config);

        let state = runtime.get_state().await;
        assert_eq!(state.height, 0);
        assert_eq!(state.supply.circulating.0, 0);
        assert_eq!(state.claimed_floors.len(), 0);
    }

    #[tokio::test]
    async fn valid_claim_mints_coins() {
        let config = NodeConfig::default();
        let runtime = NodeRuntime::new(config);

        // Set up validator
        runtime
            .mutate_state(|state| {
                state.validators = ValidatorSet::new(1);
                state.validators.add_validator(ValidatorInfo {
                    validator_id: ValidatorId([3u8; 32]),
                    public_key: [0u8; 32],
                    active: true,
                });
            })
            .await;

        let player = AccountId::new([1u8; 32]);
        let tx = sample_claim_tx(player);

        // Submit claim
        runtime
            .mutate_state(|state| {
                use crate::executor::execute_claim_floor_reward;
                let _ = execute_claim_floor_reward(state, &tx);
            })
            .await;

        // Verify state
        let state = runtime.get_state().await;
        let balance = state.balances.get(player).unwrap();
        assert_eq!(balance.mintable_coins().0, 10);
        assert_eq!(state.supply.circulating.0, 10);
    }

    #[tokio::test]
    async fn duplicate_claim_rejected() {
        let config = NodeConfig::default();
        let runtime = NodeRuntime::new(config);

        runtime
            .mutate_state(|state| {
                state.validators = ValidatorSet::new(1);
                state.validators.add_validator(ValidatorInfo {
                    validator_id: ValidatorId([3u8; 32]),
                    public_key: [0u8; 32],
                    active: true,
                });
            })
            .await;

        let player = AccountId::new([1u8; 32]);
        let tx = sample_claim_tx(player);

        // First claim
        runtime
            .mutate_state(|state| {
                use crate::executor::execute_claim_floor_reward;
                let result = execute_claim_floor_reward(state, &tx);
                assert!(result.is_ok());
            })
            .await;

        // Duplicate claim should fail
        runtime
            .mutate_state(|state| {
                use crate::executor::execute_claim_floor_reward;
                let result = execute_claim_floor_reward(state, &tx);
                assert!(result.is_err());
            })
            .await;

        // Verify supply didn't increase from second attempt
        let state = runtime.get_state().await;
        assert_eq!(state.supply.circulating.0, 10);
    }

    #[tokio::test]
    async fn balance_endpoint_returns_updated_amount() {
        let config = NodeConfig::default();
        let runtime = NodeRuntime::new(config);

        runtime
            .mutate_state(|state| {
                state.validators = ValidatorSet::new(1);
                state.validators.add_validator(ValidatorInfo {
                    validator_id: ValidatorId([3u8; 32]),
                    public_key: [0u8; 32],
                    active: true,
                });
            })
            .await;

        let player = AccountId::new([1u8; 32]);
        let tx = sample_claim_tx(player);

        runtime
            .mutate_state(|state| {
                use crate::executor::execute_claim_floor_reward;
                let _ = execute_claim_floor_reward(state, &tx);
            })
            .await;

        let state = runtime.get_state().await;
        let balance = state.balances.get(player);
        assert!(balance.is_some());
        assert_eq!(balance.unwrap().mintable_coins().0, 10);
    }

    #[tokio::test]
    async fn supply_endpoint_returns_updated_supply() {
        let config = NodeConfig::default();
        let runtime = NodeRuntime::new(config);

        runtime
            .mutate_state(|state| {
                state.validators = ValidatorSet::new(1);
                state.validators.add_validator(ValidatorInfo {
                    validator_id: ValidatorId([3u8; 32]),
                    public_key: [0u8; 32],
                    active: true,
                });
            })
            .await;

        let player = AccountId::new([1u8; 32]);
        let tx = sample_claim_tx(player);

        runtime
            .mutate_state(|state| {
                use crate::executor::execute_claim_floor_reward;
                let _ = execute_claim_floor_reward(state, &tx);
            })
            .await;

        let state = runtime.get_state().await;
        assert_eq!(state.supply.circulating.0, 10);
        assert_eq!(state.supply.burned.0, 0);
    }

    #[tokio::test]
    async fn claim_endpoint_shows_claimed_floor() {
        let config = NodeConfig::default();
        let runtime = NodeRuntime::new(config);

        runtime
            .mutate_state(|state| {
                state.validators = ValidatorSet::new(1);
                state.validators.add_validator(ValidatorInfo {
                    validator_id: ValidatorId([3u8; 32]),
                    public_key: [0u8; 32],
                    active: true,
                });
            })
            .await;

        let player = AccountId::new([1u8; 32]);
        let tx = sample_claim_tx(player);

        runtime
            .mutate_state(|state| {
                use crate::executor::execute_claim_floor_reward;
                let _ = execute_claim_floor_reward(state, &tx);
            })
            .await;

        let state = runtime.get_state().await;
        use agee_primitives::{FloorNumber};
        let claim_key = agee_primitives::ClaimKey::new(
            player,
            GameId(1),
            RunId(100),
            FloorNumber(5),
        );

        assert!(state.claimed_floors.contains(&claim_key.0));
    }
}
