#[cfg(test)]
mod tests {
    use crate::attestation::ValidatorAttestation;
    use agee_primitives::{
        AccountId, CoinAmount, ChainId, GameId, GameRulesVersion, ProtocolVersion, RunId,
        FloorNumber, ValidatorId, Hash,
    };

    fn sample_attestation() -> ValidatorAttestation {
        ValidatorAttestation {
            chain_id: ChainId([1u8; 32]),
            protocol_version: ProtocolVersion(1),
            validator_id: ValidatorId([2u8; 32]),
            player: AccountId::new([3u8; 32]),
            game_id: GameId(1),
            game_rules_version: GameRulesVersion(1),
            run_id: RunId(100),
            floor_number: FloorNumber(5),
            floor_proof_hash: Hash::new([4u8; 32]),
            calculated_reward: CoinAmount::new(10),
            reward_epoch: 1,
            validated_at_ms: 1000,
            expires_at_ms: 2000,
        }
    }

    #[test]
    fn attestation_domain_separation_is_set() {
        let attestation = sample_attestation();
        let msg = attestation.signed_message();
        assert!(msg.starts_with(b"AGEE_VALIDATOR_ATTESTATION_V1"));
    }

    #[test]
    fn different_reward_changes_signature() {
        let mut attestation_a = sample_attestation();
        let mut attestation_b = sample_attestation();

        attestation_b.calculated_reward = CoinAmount::new(20);

        let msg_a = attestation_a.signed_message();
        let msg_b = attestation_b.signed_message();

        assert_ne!(msg_a, msg_b);
    }

    #[test]
    fn different_player_changes_signature() {
        let mut attestation_a = sample_attestation();
        let mut attestation_b = sample_attestation();

        attestation_b.player = AccountId::new([99u8; 32]);

        let msg_a = attestation_a.signed_message();
        let msg_b = attestation_b.signed_message();

        assert_ne!(msg_a, msg_b);
    }

    #[test]
    fn different_chain_id_changes_signature() {
        let mut attestation_a = sample_attestation();
        let mut attestation_b = sample_attestation();

        attestation_b.chain_id = ChainId([99u8; 32]);

        let msg_a = attestation_a.signed_message();
        let msg_b = attestation_b.signed_message();

        assert_ne!(msg_a, msg_b);
    }

    #[test]
    fn different_rules_version_changes_signature() {
        let mut attestation_a = sample_attestation();
        let mut attestation_b = sample_attestation();

        attestation_b.game_rules_version = GameRulesVersion(2);

        let msg_a = attestation_a.signed_message();
        let msg_b = attestation_b.signed_message();

        assert_ne!(msg_a, msg_b);
    }

    #[test]
    fn expired_attestation_is_invalid() {
        let attestation = ValidatorAttestation {
            expires_at_ms: 100, // expired
            ..sample_attestation()
        };

        assert!(attestation.expires_at_ms < 1000);
    }
}
