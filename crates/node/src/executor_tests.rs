#[cfg(test)]
mod tests {
    use crate::executor::execute_claim_floor_reward;
    use crate::state::ChainState;
    use agee_primitives::{AccountId, CoinAmount, GameRulesVersion};
    use agee_tx::ClaimFloorReward;

    #[test]
    fn valid_claim_mints_coins() {
        let mut state = ChainState::new();
        let player = AccountId::new([1u8; 32]);

        let tx = ClaimFloorReward {
            player,
            game_id: 1,
            game_rules_version: GameRulesVersion(1),
            run_id: 100,
            floor_number: 5,
            floor_proof_hash: agee_primitives::Hash::new([2u8; 32]),
            claimed_amount: CoinAmount::new(10),
            validator_signatures: vec![vec![1, 2, 3]], // Dummy signature
        };

        let result = execute_claim_floor_reward(&mut state, &tx);
        assert!(result.is_ok());

        let balance = state.balances.get(player).unwrap();
        assert_eq!(balance.mintable_coins(), CoinAmount::new(10));
        assert_eq!(state.supply.circulating, CoinAmount::new(10));
    }

    #[test]
    fn duplicate_claim_fails() {
        let mut state = ChainState::new();
        let player = AccountId::new([1u8; 32]);

        let tx = ClaimFloorReward {
            player,
            game_id: 1,
            game_rules_version: GameRulesVersion(1),
            run_id: 100,
            floor_number: 5,
            floor_proof_hash: agee_primitives::Hash::new([2u8; 32]),
            claimed_amount: CoinAmount::new(10),
            validator_signatures: vec![vec![1, 2, 3]],
        };

        // First claim succeeds
        assert!(execute_claim_floor_reward(&mut state, &tx).is_ok());

        // Second claim with same floor fails
        let result = execute_claim_floor_reward(&mut state, &tx);
        assert!(result.is_err());
    }

    #[test]
    fn claim_over_max_supply_fails() {
        let mut state = ChainState::new();
        let player = AccountId::new([1u8; 32]);

        // Try to claim more than max supply
        let max_supply = agee_ledger::SupplyTracker::max_supply();
        let tx = ClaimFloorReward {
            player,
            game_id: 1,
            game_rules_version: GameRulesVersion(1),
            run_id: 100,
            floor_number: 5,
            floor_proof_hash: agee_primitives::Hash::new([2u8; 32]),
            claimed_amount: CoinAmount::new(max_supply.0 + 1),
            validator_signatures: vec![vec![1, 2, 3]],
        };

        let result = execute_claim_floor_reward(&mut state, &tx);
        assert!(result.is_err());
        assert_eq!(state.supply.circulating, CoinAmount::new(0));
    }

    #[test]
    fn no_validator_signatures_fails() {
        let mut state = ChainState::new();
        let player = AccountId::new([1u8; 32]);

        let tx = ClaimFloorReward {
            player,
            game_id: 1,
            game_rules_version: GameRulesVersion(1),
            run_id: 100,
            floor_number: 5,
            floor_proof_hash: agee_primitives::Hash::new([2u8; 32]),
            claimed_amount: CoinAmount::new(10),
            validator_signatures: vec![], // No signatures!
        };

        let result = execute_claim_floor_reward(&mut state, &tx);
        assert!(result.is_err());
    }
}
